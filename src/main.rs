use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame, Terminal,
};
use std::{error::Error, io, time::Duration};

struct Star {
    x: u16,
    y: u16,
    brightness: u8,
    twinkle_speed: f32,
}

struct ShootingStar {
    x: f32,
    y: f32,
    speed: f32,
    lifetime: u32,
    max_lifetime: u32,
}

impl ShootingStar {
    fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();
        ShootingStar {
            x: rng.gen_range(0..width) as f32,
            y: rng.gen_range(0..height / 2) as f32,
            speed: rng.gen_range(2.0..4.0),
            lifetime: 0,
            max_lifetime: rng.gen_range(15..30),
        }
    }

    fn update(&mut self) {
        self.x += self.speed;
        self.y += self.speed * 0.5;
        self.lifetime += 1;
    }

    fn is_alive(&self) -> bool {
        self.lifetime < self.max_lifetime
    }
}

struct Satellite {
    x: f32,
    y: f32,
    speed: f32,
    blink_phase: f32,
}

impl Satellite {
    fn new(_width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();
        Satellite {
            x: 0.0,
            y: rng.gen_range(5..height.saturating_sub(5)) as f32,
            speed: rng.gen_range(0.3..0.8),
            blink_phase: rng.gen_range(0.0..6.28),
        }
    }

    fn update(&mut self, width: u16) {
        self.x += self.speed;
        self.blink_phase += 0.1;
        
        // Reset when off screen
        if self.x > width as f32 {
            self.x = 0.0;
        }
    }
}

struct NightSky {
    stars: Vec<Star>,
    shooting_stars: Vec<ShootingStar>,
    satellites: Vec<Satellite>,
    frame_count: u32,
    width: u16,
    height: u16,
}

impl NightSky {
    fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();
        let star_count = ((width as usize * height as usize) / 20).min(300);
        
        let stars: Vec<Star> = (0..star_count)
            .map(|_| Star {
                x: rng.gen_range(0..width),
                y: rng.gen_range(0..height),
                brightness: rng.gen_range(1..=5),
                twinkle_speed: rng.gen_range(0.1..0.5),
            })
            .collect();

        // Initialize satellites (start with none, spawn randomly)
        let satellites: Vec<Satellite> = Vec::new();

        NightSky {
            stars,
            shooting_stars: Vec::new(),
            satellites,
            frame_count: 0,
            width,
            height,
        }
    }

    fn update(&mut self) {
        self.frame_count += 1;
        let mut rng = rand::thread_rng();

        // Spawn shooting stars randomly
        if rng.gen_range(0..100) < 2 {
            self.shooting_stars.push(ShootingStar::new(self.width, self.height));
        }

        // Update and remove dead shooting stars
        for star in &mut self.shooting_stars {
            star.update();
        }
        self.shooting_stars.retain(|s| s.is_alive() && s.x < self.width as f32);

        // Spawn satellites rarely (1% chance per frame, max 1 satellite)
        if self.satellites.is_empty() && rng.gen_range(0..300) < 1 {
            self.satellites.push(Satellite::new(self.width, self.height));
        }

        // Update satellites and remove those that have crossed the screen
        for satellite in &mut self.satellites {
            satellite.update(self.width);
        }
        self.satellites.retain(|s| s.x < self.width as f32);

    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        // Fill entire area with dark blue/black background using a block widget
        // This is safer than direct buffer access
        let block = Block::default()
            .style(Style::default().bg(Color::Rgb(10, 10, 30)));
        frame.render_widget(block, area);

        // Render stars
        for star in &self.stars {
            if star.x < area.width && star.y < area.height {
                // Create twinkling effect
                let twinkle = ((self.frame_count as f32 * star.twinkle_speed).sin() + 1.0) / 2.0;
                let brightness = (star.brightness as f32 * twinkle) as u8;
                
                let color = match brightness {
                    0..=1 => Color::Rgb(100, 100, 120),
                    2 => Color::Rgb(150, 150, 180),
                    3 => Color::Rgb(200, 200, 220),
                    4 => Color::Rgb(230, 230, 250),
                    _ => Color::Rgb(255, 255, 255),
                };

                let star_char = match brightness {
                    0..=1 => "·",
                    2..=3 => "•",
                    _ => "✦",
                };

                let star_widget = Paragraph::new(star_char)
                    .style(Style::default().fg(color));
                
                let star_area = Rect {
                    x: area.x + star.x,
                    y: area.y + star.y,
                    width: 1,
                    height: 1,
                };
                frame.render_widget(star_widget, star_area);
            }
        }

        // Render shooting stars
        for shooting_star in &self.shooting_stars {
            let x = shooting_star.x as u16;
            let y = shooting_star.y as u16;
            
            if x < area.width && y < area.height {
                // Main shooting star
                let star_widget = Paragraph::new("☄")
                    .style(Style::default().fg(Color::Rgb(255, 200, 100)));
                
                let star_area = Rect {
                    x: area.x + x,
                    y: area.y + y,
                    width: 1,
                    height: 1,
                };
                frame.render_widget(star_widget, star_area);
                
                // Trail
                for i in 1..4 {
                    let trail_x = (shooting_star.x - (i as f32 * 0.5)) as i32;
                    let trail_y = (shooting_star.y - (i as f32 * 0.25)) as i32;
                    
                    if trail_x >= 0 && trail_y >= 0 && (trail_x as u16) < area.width && (trail_y as u16) < area.height {
                        let trail_widget = Paragraph::new("·")
                            .style(Style::default().fg(Color::Rgb(200, 150, 50)));
                        
                        let trail_area = Rect {
                            x: area.x + trail_x as u16,
                            y: area.y + trail_y as u16,
                            width: 1,
                            height: 1,
                        };
                        frame.render_widget(trail_widget, trail_area);
                    }
                }
            }
        }

        // Render satellites
        for satellite in &self.satellites {
            let x = satellite.x as u16;
            let y = satellite.y as u16;
            
            if x < area.width && y < area.height {
                // Blinking effect
                let blink = (satellite.blink_phase.sin() + 1.0) / 2.0;
                let brightness = (200.0 + blink * 55.0) as u8;
                
                let satellite_widget = Paragraph::new("◆")
                    .style(Style::default().fg(Color::Rgb(brightness, brightness, brightness + 50)));
                
                let satellite_area = Rect {
                    x: area.x + x,
                    y: area.y + y,
                    width: 1,
                    height: 1,
                };
                frame.render_widget(satellite_widget, satellite_area);
            }
        }

    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    // Get initial terminal size
    let size = terminal.size()?;
    let mut night_sky = NightSky::new(size.width, size.height);

    let res = run_app(&mut terminal, &mut night_sky);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err)
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    night_sky: &mut NightSky,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            night_sky.render(f, f.area());
        })?;

        // Handle events with timeout for animation
        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                        return Ok(());
                    }
                }
                Event::Resize(width, height) => {
                    // Recreate night sky with new dimensions
                    *night_sky = NightSky::new(width, height);
                }
                _ => {}
            }
        }

        night_sky.update();
    }
}


use std::io::{Stdout, stdout};
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;

use crate::app::{App, AppState};
use crate::ui;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    Ok(terminal)
}

pub fn restore() -> Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub async fn run(terminal: &mut Tui, mut app: App) -> Result<()> {
    // Initial connection
    app.connect().await?;
    app.warmup().await?;

    let tick_rate = Duration::from_millis(100);
    let burst_interval = Duration::from_secs(1);
    let mut last_burst = std::time::Instant::now();

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        // Handle input with timeout
        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.disconnect().await?;
                            return Ok(());
                        }
                        KeyCode::Char(' ') => {
                            app.toggle_pause();
                        }
                        _ => {}
                    }
                }
            }
        }

        // Run bursts when not paused
        if app.state != AppState::Paused && last_burst.elapsed() >= burst_interval {
            match app.run_burst().await {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("Burst failed: {}", e);
                    app.state = AppState::Error;
                }
            }
            last_burst = std::time::Instant::now();
        }
    }
}

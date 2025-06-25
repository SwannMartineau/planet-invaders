use std::io;
use std::time::Duration;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use crate::map::tile::Tile;

pub struct AppUI {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl AppUI {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }

    pub fn run(&mut self, map: &[Vec<Tile>]) -> io::Result<()> {
        loop {
            let render_closure = |f: &mut Frame| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(100)])
                    .split(f.size());
                
                Self::render_map(f, chunks[0], map);
            };

            self.terminal.draw(render_closure)?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn render_map(f: &mut Frame, area: ratatui::layout::Rect, map: &[Vec<Tile>]) {
        let map_lines: Vec<Line> = map
            .iter()
            .map(|row| {
                let spans: Vec<Span> = row
                    .iter()
                    .map(|tile| {
                        let ch = tile.to_char();
                        let color = match tile {
                            Tile::Empty => Color::DarkGray,
                            Tile::Obstacle => Color::Gray,
                            Tile::Energy => Color::Yellow,
                            Tile::Mineral => Color::Cyan,
                            Tile::Science => Color::Magenta,
                            Tile::Base => Color::Green,
                            Tile::Robot => Color::Blue,
                        };
                        Span::styled(format!("{} ", ch), Style::default().fg(color))
                    })
                    .collect();
                Line::from(spans)
            })
            .collect();

        let map_widget = Paragraph::new(map_lines)
            .block(Block::default().borders(Borders::ALL).title("Carte - Appuyez sur 'q' pour quitter"));

        f.render_widget(map_widget, area);
    }
}

impl Drop for AppUI {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
    }
}
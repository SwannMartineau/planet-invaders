use std::io;
use std::time::Duration;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::Backend,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use crate::map::tile::Tile;
use crate::robot::Robot;

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

    pub fn render(&mut self, map: &[Vec<Tile>], robots: &[Robot]) -> io::Result<()> {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)])
                .split(f.area());
            
            // Appel de la fonction statique
            Self::render_map(f, chunks[0], map, robots);
        })?;
        Ok(())
    }

    // Fonction statique - pas besoin de &self
    fn render_map(f: &mut Frame, area: Rect, map: &[Vec<Tile>], robots: &[Robot]) {
        let mut display_map = map.to_vec();
        
        for robot in robots {
            if robot.y < display_map.len() && robot.x < display_map[robot.y].len() {
                display_map[robot.y][robot.x] = Tile::Robot;
            }
        }

        let map_lines: Vec<Line> = display_map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                let spans: Vec<Span> = row
                    .iter()
                    .enumerate()
                    .map(|(x, tile)| {
                        if let Some(robot) = robots.iter().find(|r| r.x == x && r.y == y) {
                            Span::styled(
                                format!("{} ", robot.robot_type.to_char()),
                                Style::default().fg(robot.robot_type.color())
                            )
                        } else {
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
                        }
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
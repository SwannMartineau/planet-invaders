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
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, List, ListItem},
    Frame, Terminal,
};
use crate::map::tile::Tile;
use crate::robot::{Robot, RobotType};

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
            // Diviser l'écran en deux parties : carte principale et légende
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Percentage(75), // 75% pour la carte
                    Constraint::Percentage(25), // 25% pour la légende
                ])
                .split(f.area());
            
            // Rendu de la carte
            Self::render_map(f, chunks[0], map, robots);
            
            // Rendu de la légende
            Self::render_legend(f, chunks[1]);
        })?;
        Ok(())
    }

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
                                Style::default().fg(robot.robot_type.color()).add_modifier(Modifier::BOLD)
                            )
                        } else {
                            let ch = tile.to_char();
                            let color = Self::get_tile_color(tile);
                            Span::styled(format!("{} ", ch), Style::default().fg(color))
                        }
                    })
                    .collect();
                Line::from(spans)
            })
            .collect();

        let map_widget = Paragraph::new(map_lines)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Planet Invaders - Carte")
                .border_style(Style::default().fg(Color::Green)));
        
        f.render_widget(map_widget, area);
    }

    fn render_legend(f: &mut Frame, area: Rect) {
        // Diviser la zone de légende en deux sections : Tiles et Robots
        let legend_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(60), // 60% pour les tiles
                Constraint::Percentage(40), // 40% pour les robots
            ])
            .split(area);

        // Légende des tiles
        let tile_legend_items = vec![
            Self::create_legend_item('.', Color::DarkGray, "Terrain vide"),
            Self::create_legend_item('#', Color::Gray, "Obstacle"),
            Self::create_legend_item('E', Color::Yellow, "Énergie"),
            Self::create_legend_item('M', Color::Cyan, "Minéral"),
            Self::create_legend_item('S', Color::Magenta, "Science"),
            Self::create_legend_item('B', Color::Green, "Base"),
        ];

        let tile_legend = List::new(tile_legend_items)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Terrain")
                .border_style(Style::default().fg(Color::White)));

        f.render_widget(tile_legend, legend_chunks[0]);

        // Légende des robots
        let robot_legend_items = vec![
            Self::create_legend_item('R', Color::Cyan, "Mineur"),
            Self::create_legend_item('R', Color::Yellow, "Collecteur"),
            Self::create_legend_item('R', Color::Magenta, "Scientifique"),
            Self::create_legend_item('R', Color::Green, "Explorateur"),
        ];

        let robot_legend = List::new(robot_legend_items)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Robots")
                .border_style(Style::default().fg(Color::White)));

        f.render_widget(robot_legend, legend_chunks[1]);
    }

    fn create_legend_item(symbol: char, color: Color, description: &str) -> ListItem {
        let content = Line::from(vec![
            Span::styled(
                format!("{} ", symbol),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                description,
                Style::default().fg(Color::White),
            ),
        ]);
        ListItem::new(content)
    }

    fn get_tile_color(tile: &Tile) -> Color {
        match tile {
            Tile::Empty => Color::DarkGray,
            Tile::Obstacle => Color::Gray,
            Tile::Energy => Color::Yellow,
            Tile::Mineral => Color::Cyan,
            Tile::Science => Color::Magenta,
            Tile::Base => Color::Green,
            Tile::Robot => Color::Blue,
        }
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
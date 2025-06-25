use std::io;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, List, ListItem},
    Frame, Terminal,
};
use std::collections::HashMap;
use crate::map::tile::Tile;
use crate::robot::{Robot};

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

    pub fn render(&mut self, map: &[Vec<Tile>], robots: &[Robot], base_resources: &HashMap<Tile, u32>) -> io::Result<()> {
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Percentage(75),
                    Constraint::Percentage(25),
                ])
                .split(f.area());
            Self::render_map(f, chunks[0], map, robots);
            Self::render_sidebar(f, chunks[1], base_resources);
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

    fn render_sidebar(f: &mut Frame, area: Rect, base_resources: &HashMap<Tile, u32>) {
        let sidebar_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        Self::render_base_resources(f, sidebar_chunks[0], base_resources);
        let legend_chunks = [sidebar_chunks[1], sidebar_chunks[2]];
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
    
    fn render_base_resources(f: &mut Frame, area: Rect, resources: &HashMap<Tile, u32>) {
        let resource_items = vec![
            format!("Énergie: {}", resources.get(&Tile::Energy).unwrap_or(&0)),
            format!("Minéral: {}", resources.get(&Tile::Mineral).unwrap_or(&0)),
            format!("Science: {}", resources.get(&Tile::Science).unwrap_or(&0)),
        ];
        
        let resource_lines: Vec<Line> = resource_items
            .into_iter()
            .map(|item| Line::from(Span::styled(item, Style::default().fg(Color::White))))
            .collect();
        
        let base_widget = Paragraph::new(resource_lines)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Base - Ressources")
                .border_style(Style::default().fg(Color::Green)));
        
        f.render_widget(base_widget, area);
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
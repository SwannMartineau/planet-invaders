mod map;

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

use map::tile::Tile;

fn main() -> Result<(), io::Error> {
    // Mêmes paramètres que dans ton main.rs original
    let width = 40;
    let height = 20;
    let seed = 1337;

    // Générer la map avec ta fonction existante
    let map = map::generate_map(width, height, seed);

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Boucle principale
    let res = run_app(&mut terminal, &map);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, map: &[Vec<Tile>]) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, map))?;

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

fn ui(f: &mut Frame, map: &[Vec<Tile>]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)])
        .split(f.area());

    render_map(f, chunks[0], map);
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
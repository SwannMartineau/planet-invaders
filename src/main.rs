mod map;
mod ui;
mod robot;
mod game;
mod base;

use std::io;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use game::GameState;
use ui::terminal::AppUI;

fn main() -> Result<(), io::Error> {
    let width = 80;
    let height = 80;
    let seed = 1337;

    let mut game_state = GameState::new(width, height, seed);
    let mut app_ui = AppUI::new()?;

    loop {
        game_state.update();
        app_ui.render(game_state.get_map(), game_state.get_robots())?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}
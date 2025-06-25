mod map;
mod ui;

use map::generate_map;

fn main() -> Result<(), std::io::Error> {
    // Paramètres de la carte
    let width = 40;
    let height = 20;
    let seed = 1337;

    let map = generate_map(width, height, seed);

    let mut app_ui = ui::terminal::AppUI::new()?;
    app_ui.run(&map)?;

    Ok(())
}
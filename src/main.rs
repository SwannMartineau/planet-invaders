mod map;
mod ui;
mod robot;

use std::collections::HashMap;
use std::io;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rand::Rng;
use map::generate_map;
use map::tile::Tile;
use robot::{Robot, RobotType};
use ui::terminal::AppUI;

struct GameState {
    map: Vec<Vec<Tile>>,
    robots: Vec<Robot>,
    resources: HashMap<Tile, u32>,
}

impl GameState {
    fn new(width: usize, height: usize, seed: u32) -> Self {
        let map = generate_map(width, height, seed);
        
        // Configuration des robots
        let robot_counts = vec![
            (RobotType::Miner, 0),      // 3 robots mineurs
            (RobotType::EnergyCollector, 0), // 3 robots collecteurs d'énergie
            (RobotType::Scientist, 0),  // 2 robots scientifiques
            (RobotType::Explorer, 20),   // 2 robots explorateurs
        ];

        let base_pos = find_base_position(&map).expect("Base not found");
        let robots = spawn_robots_near_base(base_pos, &robot_counts, &map);

        let mut resources = HashMap::new();
        resources.insert(Tile::Mineral, 0);
        resources.insert(Tile::Energy, 0);
        resources.insert(Tile::Science, 0);

        Self { map, robots, resources }
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();
        
        for robot in &mut self.robots {
            // Déplacement aléatoire (à remplacer par la logique spécifique)
            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            let (dx, dy) = directions[rng.gen_range(0..directions.len())];
            
            let new_x = (robot.x as isize + dx) as usize;
            let new_y = (robot.y as isize + dy) as usize;
            
            if robot.can_move_to(new_x, new_y, &self.map) {
                // Mettre à jour la position
                robot.x = new_x;
                robot.y = new_y;
                
                // Collecter les ressources si nécessaire
                if self.map[new_y][new_x].is_consumable() {
                    robot.collect(self.map[new_y][new_x]);
                    *self.resources.get_mut(&self.map[new_y][new_x]).unwrap() += 1;
                    self.map[new_y][new_x] = Tile::Empty;
                }
            }
        }
    }
}

fn find_base_position(map: &[Vec<Tile>]) -> Option<(usize, usize)> {
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::Base {
                return Some((x, y));
            }
        }
    }
    None
}

fn spawn_robots_near_base(
    base_pos: (usize, usize),
    robot_counts: &[(RobotType, usize)],
    map: &[Vec<Tile>],
) -> Vec<Robot> {
    let (base_x, base_y) = base_pos;
    let mut robots = Vec::new();
    let mut positions = Vec::new();

    // Générer les positions valides autour de la base
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue; // Skip la position de la base elle-même
            }
            let x = (base_x as isize + dx) as usize;
            let y = (base_y as isize + dy) as usize;
            
            if y < map.len() && x < map[y].len() && map[y][x] == Tile::Empty {
                positions.push((x, y));
            }
        }
    }

    // Créer les robots sur les positions disponibles
    for (robot_type, count) in robot_counts {
        for _ in 0..*count {
            if let Some((x, y)) = positions.pop() {
                robots.push(Robot::new(x, y, *robot_type));
            }
        }
    }

    robots
}

fn main() -> Result<(), io::Error> {
    // Configuration de la carte
    let width = 40;
    let height = 20;
    let seed = 1337;

    // Initialisation du jeu
    let mut game_state = GameState::new(width, height, seed);
    let mut app_ui = AppUI::new()?; // Correction ici

    // Boucle principale du jeu
    loop {
        // Mise à jour de l'état du jeu
        game_state.update();

        // Affichage
        app_ui.render(&game_state.map, &game_state.robots)?;

        // Gestion des inputs
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
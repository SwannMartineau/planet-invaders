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
use robot::{Robot, RobotType, RobotState};
use ui::terminal::AppUI;

#[derive(Debug, Clone)]
struct DiscoveredResource {
    x: usize,
    y: usize,
    tile_type: Tile,
    assigned_robot_id: Option<usize>,
}

struct GameState {
    map: Vec<Vec<Tile>>,
    robots: Vec<Robot>,
    resources: HashMap<Tile, u32>,
    discovered_resources: Vec<DiscoveredResource>,
    base_position: (usize, usize),
}

impl GameState {
    fn new(width: usize, height: usize, seed: u32) -> Self {
        let map = generate_map(width, height, seed);
        
        // Configuration des robots
        let robot_counts = vec![
            (RobotType::Explorer, 5),
            (RobotType::Miner, 2),
            (RobotType::EnergyCollector, 2),
            (RobotType::Scientist, 1),
        ];

        let base_positions = find_all_base_positions(&map);
        let robots = spawn_robots_in_base(&base_positions, &robot_counts);

        let mut resources = HashMap::new();
        resources.insert(Tile::Mineral, 0);
        resources.insert(Tile::Energy, 0);
        resources.insert(Tile::Science, 0);

        Self { 
            map, 
            robots, 
            resources, 
            discovered_resources: Vec::new(),
            base_position: base_positions[0],
        }
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();
        let mut new_discoveries = Vec::new();
        
        for robot in self.robots.iter_mut() {
            if robot.robot_type == RobotType::Explorer {
                let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                let (dx, dy) = directions[rng.gen_range(0..directions.len())];
                
                let new_x = (robot.x as isize + dx) as usize;
                let new_y = (robot.y as isize + dy) as usize;
                
                if robot.can_move_to(new_x, new_y, &self.map) {
                    robot.x = new_x;
                    robot.y = new_y;
                    
                    let current_tile = self.map[new_y][new_x];
                    if current_tile.is_consumable() {
                        let already_discovered = self.discovered_resources.iter()
                            .any(|res| res.x == new_x && res.y == new_y);
                        
                        if !already_discovered {
                            robot.record_exploration(new_x, new_y, current_tile);
                            new_discoveries.push(DiscoveredResource {
                                x: new_x,
                                y: new_y,
                                tile_type: current_tile,
                                assigned_robot_id: None,
                            });
                        }
                    }
                }
            }
        }
        
        self.discovered_resources.extend(new_discoveries);
        
        self.assign_resources_to_collectors();
        
        let assigned_resources: Vec<(usize, usize, usize)> = self.discovered_resources.iter()
            .filter_map(|res| res.assigned_robot_id.map(|id| (id, res.x, res.y)))
            .collect();
        
        let mut resources_to_remove = Vec::new();
        let base_pos = self.base_position;
        
        for (robot_id, robot) in self.robots.iter_mut().enumerate() {
            if robot.robot_type != RobotType::Explorer {
                match robot.state {
                    RobotState::Idle => {
                    },
                    RobotState::GoingToResource => {
                        if let Some((_, target_x, target_y)) = assigned_resources.iter()
                            .find(|(id, _, _)| *id == robot_id) {
                            
                            let target_x = *target_x;
                            let target_y = *target_y;
                            
                            if robot.x == target_x && robot.y == target_y {
                                let tile = self.map[target_y][target_x];
                                if robot.can_collect(tile) {
                                    robot.collect(tile);
                                    self.map[target_y][target_x] = Tile::Empty;
                                    
                                    resources_to_remove.push((target_x, target_y));
                                    
                                    robot.set_returning_to_base(base_pos.0, base_pos.1);
                                }
                            } else {
                                robot.move_toward(target_x, target_y, &self.map);
                            }
                        }
                    },
                    RobotState::ReturningToBase => {
                        if robot.x == base_pos.0 && robot.y == base_pos.1 {
                            let unloaded_items = robot.unload_inventory();
                            for item in unloaded_items {
                                *self.resources.get_mut(&item).unwrap() += 1;
                            }
                        } else {
                            robot.move_toward(base_pos.0, base_pos.1, &self.map);
                        }
                    }
                }
            }
        }
        
        for (x, y) in resources_to_remove {
            self.discovered_resources.retain(|res| !(res.x == x && res.y == y));
        }
    }
    
    fn assign_resources_to_collectors(&mut self) {
        let mut available_miners = Vec::new();
        let mut available_energy_collectors = Vec::new();
        let mut available_scientists = Vec::new();
        
        for (robot_id, robot) in self.robots.iter().enumerate() {
            let already_assigned = self.discovered_resources.iter()
                .any(|res| res.assigned_robot_id == Some(robot_id));
            
            if !already_assigned && robot.is_idle() {
                match robot.robot_type {
                    RobotType::Miner => available_miners.push(robot_id),
                    RobotType::EnergyCollector => available_energy_collectors.push(robot_id),
                    RobotType::Scientist => available_scientists.push(robot_id),
                    RobotType::Explorer => {},
                }
            }
        }
        
        for resource in &mut self.discovered_resources {
            if resource.assigned_robot_id.is_none() {
                let robot_id = match resource.tile_type {
                    Tile::Mineral => available_miners.pop(),
                    Tile::Energy => available_energy_collectors.pop(),
                    Tile::Science => available_scientists.pop(),
                    _ => None,
                };
                
                if let Some(id) = robot_id {
                    resource.assigned_robot_id = Some(id);
                    if let Some(robot) = self.robots.get_mut(id) {
                        robot.set_target(resource.x, resource.y);
                    }
                }
            }
        }
    }
}

fn find_all_base_positions(map: &[Vec<Tile>]) -> Vec<(usize, usize)> {
    let mut base_positions = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::Base {
                base_positions.push((x, y));
            }
        }
    }
    if base_positions.is_empty() {
        panic!("Aucune base trouvée sur la carte !");
    }
    base_positions
}

fn spawn_robots_in_base(
    base_positions: &[(usize, usize)],
    robot_counts: &[(RobotType, usize)],
) -> Vec<Robot> {
    let mut robots = Vec::new();
    let mut position_index = 0;

    println!("Positions de base disponibles: {}", base_positions.len());

    for (robot_type, count) in robot_counts {
        for i in 0..*count {
            if position_index < base_positions.len() {
                let (x, y) = base_positions[position_index];
                robots.push(Robot::new(x, y, *robot_type));
                println!("Robot créé: {:?} à la position de base ({}, {})", robot_type, x, y);
                position_index += 1;
            } else {
                position_index = 0;
                let (x, y) = base_positions[position_index];
                robots.push(Robot::new(x, y, *robot_type));
                println!("Robot créé: {:?} à la position de base ({}, {}) - Position réutilisée", robot_type, x, y);
                position_index += 1;
            }
        }
    }

    println!("Total des robots créés: {} dans {} positions de base", 
             robots.len(), 
             base_positions.len());

    robots
}

fn main() -> Result<(), io::Error> {
    let width = 80;
    let height = 80;
    let seed = 1337;

    let mut game_state = GameState::new(width, height, seed);
    let mut app_ui = AppUI::new()?;

    loop {
        game_state.update();

        app_ui.render(&game_state.map, &game_state.robots)?;

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
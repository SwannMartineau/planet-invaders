use std::collections::HashMap;
use crate::map::tile::Tile;
use crate::robot::{Robot, RobotType};

#[derive(Clone, Debug)]
pub struct Base {
    pub x: usize,
    pub y: usize,
    pub inventory: HashMap<Tile, u32>,
}

impl Base {
    pub fn new(map_width: usize, map_height: usize) -> Self {
        let size = usize::min(10, usize::min(map_width, map_height) / 5);
        let x = (map_width - size) / 2;
        let y = (map_height - size) / 2;
        
        let mut inventory = HashMap::new();
        inventory.insert(Tile::Mineral, 0);
        inventory.insert(Tile::Energy, 0);
        inventory.insert(Tile::Science, 0);
        
        Self { x, y, inventory }
    }
    
    pub fn add_resource(&mut self, resource: Tile) {
        *self.inventory.get_mut(&resource).unwrap() += 1;
    }
    
    pub fn get_resources(&self) -> &HashMap<Tile, u32> {
        &self.inventory
    }
}

pub fn find_all_base_positions(map: &[Vec<Tile>]) -> Vec<(usize, usize)> {
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

pub fn spawn_robots_in_base(
    base_positions: &[(usize, usize)],
    robot_counts: &[(RobotType, usize)],
) -> Vec<Robot> {
    let mut robots = Vec::new();
    let mut position_index = 0;

    println!("Positions de base disponibles: {}", base_positions.len());

    for (robot_type, count) in robot_counts {
        for _i in 0..*count {
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
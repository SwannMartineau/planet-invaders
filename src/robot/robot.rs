use crate::map::tile::Tile;
use super::types::RobotType;
use std::collections::{VecDeque, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum RobotState {
    Idle,
    GoingToResource,
    ReturningToBase,
}

#[derive(Debug, Clone)]
pub struct Robot {
    pub x: usize,
    pub y: usize,
    pub robot_type: RobotType,
    pub inventory: Vec<Tile>,
    pub explored_tiles: Vec<(usize, usize, Tile)>,
    current_target: Option<(usize, usize)>,
    pub state: RobotState,
    pub path: Vec<(usize, usize)>, // Chemin calculé vers la cible
    pub stuck_counter: u32, // Compteur pour détecter si le robot est bloqué
    pub last_position: Option<(usize, usize)>, // Dernière position pour détecter le blocage
}

impl Robot {
    pub fn new(x: usize, y: usize, robot_type: RobotType) -> Self {
        Self {
            x,
            y,
            robot_type,
            inventory: Vec::new(),
            explored_tiles: Vec::new(),
            current_target: None,
            state: RobotState::Idle,
            path: Vec::new(),
            stuck_counter: 0,
            last_position: None,
        }
    }

    pub fn move_to(&mut self, new_x: usize, new_y: usize) {
        self.x = new_x;
        self.y = new_y;
    }

    pub fn collect(&mut self, tile: Tile) {
        if self.can_collect(tile) {
            self.inventory.push(tile);
        }
    }

    pub fn can_collect(&self, tile: Tile) -> bool {
        matches!((self.robot_type, tile), 
            (RobotType::Miner, Tile::Mineral) |
            (RobotType::EnergyCollector, Tile::Energy) |
            (RobotType::Scientist, Tile::Science)
        )
    }

    pub fn record_exploration(&mut self, x: usize, y: usize, tile: Tile) {
        if !self.explored_tiles.iter().any(|(ex, ey, _)| *ex == x && *ey == y) {
            self.explored_tiles.push((x, y, tile));
        }
    }

    pub fn set_target(&mut self, target_x: usize, target_y: usize) {
        self.current_target = Some((target_x, target_y));
        if self.state == RobotState::Idle {
            self.state = RobotState::GoingToResource;
        }
        self.path.clear();
    }


    pub fn set_returning_to_base(&mut self, base_x: usize, base_y: usize) {
        self.current_target = Some((base_x, base_y));
        self.state = RobotState::ReturningToBase;
        self.path.clear();
    }

    pub fn is_idle(&self) -> bool {
        self.state == RobotState::Idle
    }

    pub fn unload_inventory(&mut self) -> Vec<Tile> {
        let items = self.inventory.clone();
        self.inventory.clear();
        self.state = RobotState::Idle;
        self.current_target = None;
        self.path.clear();
        self.stuck_counter = 0;
        items
    }

    pub fn move_toward(&mut self, target_x: usize, target_y: usize, map: &[Vec<Tile>]) -> bool {
        if let Some((last_x, last_y)) = self.last_position {
            if last_x == self.x && last_y == self.y {
                self.stuck_counter += 1;
            } else {
                self.stuck_counter = 0;
            }
        }
        self.last_position = Some((self.x, self.y));

        if self.stuck_counter > 5 {
            self.path.clear();
            self.stuck_counter = 0;
        }

        if self.path.is_empty() {
            self.path = self.find_path_to(target_x, target_y, map);
        }

        if let Some(&(next_x, next_y)) = self.path.first() {
            if self.can_move_to(next_x, next_y, map) {
                self.x = next_x;
                self.y = next_y;
                self.path.remove(0);
                return true;
            } else {
                self.path.clear();
                return false;
            }
        }

        self.simple_move_toward(target_x, target_y, map)
    }

    fn find_path_to(&self, target_x: usize, target_y: usize, map: &[Vec<Tile>]) -> Vec<(usize, usize)> {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from = std::collections::HashMap::new();
        
        let start = (self.x, self.y);
        let target = (target_x, target_y);
        
        queue.push_back(start);
        visited.insert(start);
        
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        
        while let Some((x, y)) = queue.pop_front() {
            if (x, y) == target {
                let mut path = Vec::new();
                let mut current = target;
                
                while current != start {
                    path.push(current);
                    current = came_from[&current];
                }
                
                path.reverse();
                return path;
            }
            
            for (dx, dy) in directions {
                let new_x = (x as isize + dx) as usize;
                let new_y = (y as isize + dy) as usize;
                
                if new_x < map[0].len() && new_y < map.len() {
                    let pos = (new_x, new_y);
                    
                    if !visited.contains(&pos) && self.can_move_to(new_x, new_y, map) {
                        visited.insert(pos);
                        came_from.insert(pos, (x, y));
                        queue.push_back(pos);
                    }
                }
            }
        }
        
        Vec::new()
    }

    fn simple_move_toward(&mut self, target_x: usize, target_y: usize, map: &[Vec<Tile>]) -> bool {
        let dx = target_x as isize - self.x as isize;
        let dy = target_y as isize - self.y as isize;

        let mut possible_moves = Vec::new();
        
        if dx != 0 {
            let new_x = (self.x as isize + dx.signum()) as usize;
            if self.can_move_to(new_x, self.y, map) {
                possible_moves.push((new_x, self.y));
            }
        }
        
        if dy != 0 {
            let new_y = (self.y as isize + dy.signum()) as usize;
            if self.can_move_to(self.x, new_y, map) {
                possible_moves.push((self.x, new_y));
            }
        }

        if possible_moves.is_empty() {
            let directions = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];
            for (dx, dy) in directions {
                let new_x = (self.x as isize + dx) as usize;
                let new_y = (self.y as isize + dy) as usize;
                if self.can_move_to(new_x, new_y, map) {
                    possible_moves.push((new_x, new_y));
                }
            }
        }
        
        if let Some(&(new_x, new_y)) = possible_moves.iter()
            .min_by_key(|&&(x, y)| {
                let dx = target_x as isize - x as isize;
                let dy = target_y as isize - y as isize;
                dx * dx + dy * dy
            }) {
            self.x = new_x;
            self.y = new_y;
            return true;
        }

        false
    }

    pub fn can_move_to(&self, x: usize, y: usize, map: &[Vec<Tile>]) -> bool {
        if y >= map.len() || x >= map[y].len() {
            return false;
        }
        matches!(map[y][x], 
            Tile::Empty | Tile::Energy | Tile::Mineral | Tile::Science | Tile::Base
        )
    }

    pub fn is_collector(&self) -> bool {
        matches!(self.robot_type, RobotType::Miner | RobotType::EnergyCollector | RobotType::Scientist)
    }

    pub fn is_explorer(&self) -> bool {
        self.robot_type == RobotType::Explorer
    }
}
use crate::map::tile::Tile;
use super::types::RobotType;

#[derive(Debug, Clone, PartialEq)]
pub enum RobotState {
    Idle,           // En attente d'une mission
    GoingToResource, // Se dirigeant vers une ressource
    ReturningToBase, // Retournant à la base avec des ressources
}

#[derive(Debug, Clone)]
pub struct Robot {
    pub x: usize,
    pub y: usize,
    pub robot_type: RobotType,
    pub inventory: Vec<Tile>,
    pub explored_tiles: Vec<(usize, usize, Tile)>,
    pub state: RobotState,
    current_target: Option<(usize, usize)>,
}

impl Robot {
    pub fn new(x: usize, y: usize, robot_type: RobotType) -> Self {
        Self {
            x,
            y,
            robot_type,
            inventory: Vec::new(),
            explored_tiles: Vec::new(),
            state: RobotState::Idle,
            current_target: None,
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
    }

    pub fn set_returning_to_base(&mut self, base_x: usize, base_y: usize) {
        self.current_target = Some((base_x, base_y));
        self.state = RobotState::ReturningToBase;
    }

    pub fn is_idle(&self) -> bool {
        self.state == RobotState::Idle
    }

    pub fn unload_inventory(&mut self) -> Vec<Tile> {
        let items = self.inventory.clone();
        self.inventory.clear();
        self.state = RobotState::Idle;
        self.current_target = None;
        items
    }

    pub fn move_toward(&mut self, target_x: usize, target_y: usize, map: &[Vec<Tile>]) -> bool {
        let dx = target_x as isize - self.x as isize;
        let dy = target_y as isize - self.y as isize;

        // Déplacement prioritaire sur l'axe avec la plus grande distance
        if dx.abs() > dy.abs() {
            let new_x = (self.x as isize + dx.signum()) as usize;
            if self.can_move_to(new_x, self.y, map) {
                self.x = new_x;
                return true;
            } else if dy != 0 {
                let new_y = (self.y as isize + dy.signum()) as usize;
                if self.can_move_to(self.x, new_y, map) {
                    self.y = new_y;
                    return true;
                }
            }
        } else if dy != 0 {
            let new_y = (self.y as isize + dy.signum()) as usize;
            if self.can_move_to(self.x, new_y, map) {
                self.y = new_y;
                return true;
            } else if dx != 0 {
                let new_x = (self.x as isize + dx.signum()) as usize;
                if self.can_move_to(new_x, self.y, map) {
                    self.x = new_x;
                    return true;
                }
            }
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
        matches!(self.robot_type, 
            RobotType::Miner | RobotType::EnergyCollector | RobotType::Scientist
        )
    }

    pub fn is_explorer(&self) -> bool {
        self.robot_type == RobotType::Explorer
    }
}
use crate::map::tile::Tile;
use super::types::RobotType;

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
    pub current_target: Option<(usize, usize)>,
    pub state: RobotState,
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
        match (self.robot_type, tile) {
            (RobotType::Miner, Tile::Mineral) => true,
            (RobotType::EnergyCollector, Tile::Energy) => true,
            (RobotType::Scientist, Tile::Science) => true,
            _ => false,
        }
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

    pub fn clear_target(&mut self) {
        self.current_target = None;
        self.state = RobotState::Idle;
    }

    pub fn set_returning_to_base(&mut self, base_x: usize, base_y: usize) {
        self.current_target = Some((base_x, base_y));
        self.state = RobotState::ReturningToBase;
    }

    pub fn is_returning_to_base(&self) -> bool {
        self.state == RobotState::ReturningToBase
    }

    pub fn is_idle(&self) -> bool {
        self.state == RobotState::Idle
    }

    pub fn is_going_to_resource(&self) -> bool {
        self.state == RobotState::GoingToResource
    }

    pub fn has_inventory(&self) -> bool {
        !self.inventory.is_empty()
    }

    pub fn unload_inventory(&mut self) -> Vec<Tile> {
        let items = self.inventory.clone();
        self.inventory.clear();
        self.state = RobotState::Idle;
        self.current_target = None;
        items
    }

    pub fn has_target(&self) -> bool {
        self.current_target.is_some()
    }

    pub fn get_target(&self) -> Option<(usize, usize)> {
        self.current_target
    }

    pub fn is_at_target(&self) -> bool {
        if let Some((target_x, target_y)) = self.current_target {
            self.x == target_x && self.y == target_y
        } else {
            false
        }
    }

    pub fn move_toward(&mut self, target_x: usize, target_y: usize, map: &[Vec<Tile>]) -> bool {
        let dx = target_x as isize - self.x as isize;
        let dy = target_y as isize - self.y as isize;

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
        match map[y][x] {
            Tile::Empty | Tile::Energy | Tile::Mineral | Tile::Science | Tile::Base => true,
            Tile::Obstacle | Tile::Robot => false,
        }
    }

    pub fn distance_to(&self, target_x: usize, target_y: usize) -> f64 {
        let dx = target_x as f64 - self.x as f64;
        let dy = target_y as f64 - self.y as f64;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn get_compatible_resource_types(&self) -> Vec<Tile> {
        match self.robot_type {
            RobotType::Miner => vec![Tile::Mineral],
            RobotType::EnergyCollector => vec![Tile::Energy],
            RobotType::Scientist => vec![Tile::Science],
            RobotType::Explorer => vec![],
        }
    }

    pub fn is_collector(&self) -> bool {
        matches!(self.robot_type, RobotType::Miner | RobotType::EnergyCollector | RobotType::Scientist)
    }

    pub fn is_explorer(&self) -> bool {
        self.robot_type == RobotType::Explorer
    }
}
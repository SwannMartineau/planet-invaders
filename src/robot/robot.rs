use crate::map::tile::Tile;
use super::types::RobotType;

#[derive(Debug, Clone)]
pub struct Robot {
    pub x: usize,
    pub y: usize,
    pub robot_type: RobotType,
    pub inventory: Vec<Tile>,
    pub explored_tiles: Vec<(usize, usize, Tile)>,
}

impl Robot {
    pub fn new(x: usize, y: usize, robot_type: RobotType) -> Self {
        Self {
            x,
            y,
            robot_type,
            inventory: Vec::new(),
            explored_tiles: Vec::new(),
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
        self.explored_tiles.push((x, y, tile));
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
        } else {
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
            Tile::Empty | Tile::Energy | Tile::Mineral | Tile::Science => true,
            Tile::Obstacle | Tile::Base | Tile::Robot => false,
        }
    }
}

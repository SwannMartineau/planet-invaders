use std::collections::{HashMap, HashSet};
use rand::Rng;
use crate::map::{generate_map, tile::Tile};
use crate::robot::{Robot, RobotType, RobotState};

#[derive(Debug, Clone)]
pub struct DiscoveredResource {
    pub x: usize,
    pub y: usize,
    pub tile_type: Tile,
    pub assigned_robot_id: Option<usize>,
}

pub struct GameState {
    map: Vec<Vec<Tile>>,
    robots: Vec<Robot>,
    resources: HashMap<Tile, u32>,
    discovered_resources: Vec<DiscoveredResource>,
    base_position: (usize, usize),
}

impl GameState {
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        let map = generate_map(width, height, seed);
        
        // Configuration des robots
        let robot_counts = vec![
            (RobotType::Explorer, 20),
            (RobotType::Miner, 10),
            (RobotType::EnergyCollector, 10),
            (RobotType::Scientist, 5),
        ];

        let base_positions = Self::find_all_base_positions(&map);
        let robots = Self::spawn_robots_in_base(&base_positions, &robot_counts);

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

    pub fn get_map(&self) -> &[Vec<Tile>] {
        &self.map
    }

    pub fn get_robots(&self) -> &[Robot] {
        &self.robots
    }

    pub fn update(&mut self) {
        self.update_explorers();
        self.assign_resources_to_collectors();
        self.update_collectors();
        self.cleanup_collected_resources();
    }

    fn update_explorers(&mut self) {
        let mut rng = rand::thread_rng();
        let mut new_discoveries = Vec::new();
        
        // Collecter d'abord les positions déjà découvertes pour éviter les conflits d'emprunt
        let discovered_positions: std::collections::HashSet<(usize, usize)> = 
            self.discovered_resources.iter().map(|res| (res.x, res.y)).collect();
        
        for robot in self.robots.iter_mut().filter(|r| r.robot_type == RobotType::Explorer) {
            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            let (dx, dy) = directions[rng.gen_range(0..directions.len())];
            
            let new_x = (robot.x as isize + dx) as usize;
            let new_y = (robot.y as isize + dy) as usize;
            
            if robot.can_move_to(new_x, new_y, &self.map) {
                robot.move_to(new_x, new_y);
                
                let current_tile = self.map[new_y][new_x];
                if Self::is_resource_tile(current_tile) && !discovered_positions.contains(&(new_x, new_y)) {
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
        
        self.discovered_resources.extend(new_discoveries);
    }

    fn update_collectors(&mut self) {
        let assigned_resources: Vec<(usize, usize, usize)> = self.discovered_resources.iter()
            .filter_map(|res| res.assigned_robot_id.map(|id| (id, res.x, res.y)))
            .collect();
        
        let mut resources_to_remove = Vec::new();
        let base_pos = self.base_position;
        
        for (robot_id, robot) in self.robots.iter_mut().enumerate().filter(|(_, r)| r.robot_type != RobotType::Explorer) {
            match robot.state {
                RobotState::Idle => {
                    // Robot en attente
                },
                RobotState::GoingToResource => {
                    if let Some((_, target_x, target_y)) = assigned_resources.iter().find(|(id, _, _)| *id == robot_id) {
                        let target_x = *target_x;
                        let target_y = *target_y;
                        
                        if robot.x == target_x && robot.y == target_y {
                            let tile = self.map[target_y][target_x];
                            
                            if robot.can_collect(tile) {
                                robot.collect(tile);
                                self.map[target_y][target_x] = Tile::Empty;
                                resources_to_remove.push((target_x, target_y));
                                robot.set_returning_to_base(base_pos.0, base_pos.1);
                            } else {
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
        
        // Stocker les ressources à supprimer pour nettoyage
        for (x, y) in resources_to_remove {
            self.cleanup_resource_at(x, y);
        }
    }

    fn assign_resources_to_collectors(&mut self) {
        let available_robots = self.get_available_robots_by_type();
        
        // Créer une liste des assignments à effectuer pour éviter les conflits d'emprunt
        let mut assignments = Vec::new();
        
        for (index, resource) in self.discovered_resources.iter().enumerate() {
            if resource.assigned_robot_id.is_none() {
                let robot_type = match resource.tile_type {
                    Tile::Mineral => RobotType::Miner,
                    Tile::Energy => RobotType::EnergyCollector,
                    Tile::Science => RobotType::Scientist,
                    _ => continue,
                };
                
                if let Some(robot_list) = available_robots.get(&robot_type) {
                    if let Some(&robot_id) = robot_list.first() {
                        assignments.push((index, robot_id, resource.x, resource.y));
                    }
                }
            }
        }
        
        // Appliquer les assignments
        for (resource_index, robot_id, target_x, target_y) in assignments {
            if let Some(resource) = self.discovered_resources.get_mut(resource_index) {
                resource.assigned_robot_id = Some(robot_id);
            }
            if let Some(robot) = self.robots.get_mut(robot_id) {
                robot.set_target(target_x, target_y);
            }
        }
    }

    fn get_available_robots_by_type(&self) -> HashMap<RobotType, Vec<usize>> {
        let mut available = HashMap::new();
        available.insert(RobotType::Miner, Vec::new());
        available.insert(RobotType::EnergyCollector, Vec::new());
        available.insert(RobotType::Scientist, Vec::new());
        
        for (robot_id, robot) in self.robots.iter().enumerate() {
            let already_assigned = self.discovered_resources.iter()
                .any(|res| res.assigned_robot_id == Some(robot_id));
            
            if !already_assigned && robot.is_idle() && robot.robot_type != RobotType::Explorer {
                if let Some(robot_list) = available.get_mut(&robot.robot_type) {
                    robot_list.push(robot_id);
                }
            }
        }
        
        available
    }

    fn cleanup_resource_at(&mut self, x: usize, y: usize) {
        self.discovered_resources.retain(|res| !(res.x == x && res.y == y));
    }

    fn cleanup_collected_resources(&mut self) {
        // Cette méthode est appelée à la fin d'update() mais le nettoyage
        // est déjà fait dans update_collectors() pour éviter les conflits d'emprunt
    }

    // Méthode helper pour identifier les ressources de façon fiable
    fn is_resource_tile(tile: Tile) -> bool {
        matches!(tile, Tile::Mineral | Tile::Energy | Tile::Science)
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

        for (robot_type, count) in robot_counts {
            for _i in 0..*count {
                if position_index < base_positions.len() {
                    let (x, y) = base_positions[position_index];
                    robots.push(Robot::new(x, y, *robot_type));
                    
                    position_index += 1;
                } else {
                    position_index = 0;
                    let (x, y) = base_positions[position_index];
                    robots.push(Robot::new(x, y, *robot_type));
                    position_index += 1;
                }
            }
        }

        robots
    }
}
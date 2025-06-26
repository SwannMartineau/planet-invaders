#[cfg(test)]
mod robot_tests {
    use super::super::{Robot, RobotType, RobotState};
    use crate::map::tile::Tile;

    #[test]
    fn test_robot_creation() {
        let robot = Robot::new(10, 15, RobotType::Explorer);
        
        assert_eq!(robot.x, 10);
        assert_eq!(robot.y, 15);
        assert_eq!(robot.robot_type, RobotType::Explorer);
        assert_eq!(robot.state, RobotState::Idle);
        assert!(robot.inventory.is_empty());
    }

    #[test]
    fn test_robot_move_to() {
        let mut robot = Robot::new(5, 5, RobotType::Miner);
        robot.move_to(8, 12);
        
        assert_eq!(robot.x, 8);
        assert_eq!(robot.y, 12);
    }

    #[test]
    fn test_robot_can_move_to() {
        let robot = Robot::new(0, 0, RobotType::Explorer);
        let map = vec![
            vec![Tile::Empty, Tile::Obstacle],
            vec![Tile::Base, Tile::Empty],
        ];
        
        assert!(robot.can_move_to(0, 1, &map));
        assert!(robot.can_move_to(1, 1, &map));
        assert!(!robot.can_move_to(1, 0, &map));
        assert!(!robot.can_move_to(2, 0, &map));
    }

    #[test]
    fn test_robot_can_collect() {
        let miner = Robot::new(0, 0, RobotType::Miner);
        let energy_collector = Robot::new(0, 0, RobotType::EnergyCollector);
        let scientist = Robot::new(0, 0, RobotType::Scientist);
        
        assert!(miner.can_collect(Tile::Mineral));
        assert!(!miner.can_collect(Tile::Energy));
        assert!(!miner.can_collect(Tile::Science));
        
        assert!(energy_collector.can_collect(Tile::Energy));
        assert!(!energy_collector.can_collect(Tile::Mineral));
        
        assert!(scientist.can_collect(Tile::Science));
        assert!(!scientist.can_collect(Tile::Mineral));
    }

    #[test]
    fn test_robot_unload_inventory() {
        let mut robot = Robot::new(0, 0, RobotType::Miner);
        robot.collect(Tile::Mineral);
        robot.collect(Tile::Mineral);
        
        let unloaded = robot.unload_inventory();
        
        assert_eq!(unloaded.len(), 2);
        assert!(robot.inventory.is_empty());
        assert_eq!(robot.state, RobotState::Idle);
    }

    #[test]
    fn test_robot_set_target() {
        let mut robot = Robot::new(0, 0, RobotType::Miner);
        robot.set_target(10, 15);
        
        assert_eq!(robot.state, RobotState::GoingToResource);
    }

    #[test]
    fn test_robot_set_returning_to_base() {
        let mut robot = Robot::new(0, 0, RobotType::Miner);
        robot.set_returning_to_base(5, 8);
        
        assert_eq!(robot.state, RobotState::ReturningToBase);
    }

    #[test]
    fn test_robot_is_idle() {
        let idle_robot = Robot::new(0, 0, RobotType::Explorer);
        let mut working_robot = Robot::new(0, 0, RobotType::Miner);
        working_robot.set_target(10, 10);
        
        assert!(idle_robot.is_idle());
        assert!(!working_robot.is_idle());
    }
}
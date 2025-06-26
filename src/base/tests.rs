#[cfg(test)]
mod base_tests {
    use super::super::base::{Base, find_all_base_positions, spawn_robots_in_base};
    use crate::map::tile::Tile;
    use crate::robot::RobotType;

    #[test]
    fn test_base_creation() {
        let base = Base::new(100, 80);
        
        assert_eq!(base.x, 45);
        assert_eq!(base.y, 35);
        assert_eq!(base.inventory[&Tile::Energy], 0);
        assert_eq!(base.inventory[&Tile::Mineral], 0);
        assert_eq!(base.inventory[&Tile::Science], 0);
    }

    #[test]
    fn test_base_small_map() {
        let base = Base::new(20, 15);
        
        assert_eq!(base.x, 8);
        assert_eq!(base.y, 6);
    }

    #[test]
    fn test_add_resources() {
        let mut base = Base::new(50, 50);
        
        base.add_resource(Tile::Energy);
        base.add_resource(Tile::Energy);
        base.add_resource(Tile::Mineral);
        
        assert_eq!(base.inventory[&Tile::Energy], 2);
        assert_eq!(base.inventory[&Tile::Mineral], 1);
        assert_eq!(base.inventory[&Tile::Science], 0);
    }

    #[test]
    fn test_find_base_positions() {
        let map = vec![
            vec![Tile::Empty, Tile::Base, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Base, Tile::Empty, Tile::Base],
        ];
        
        let positions = find_all_base_positions(&map);
        
        assert_eq!(positions.len(), 3);
        assert!(positions.contains(&(1, 0)));
        assert!(positions.contains(&(0, 2)));
        assert!(positions.contains(&(2, 2)));
    }

    #[test]
    #[should_panic(expected = "Aucune base trouvée sur la carte !")]
    fn test_no_base_panic() {
        let map = vec![
            vec![Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty],
        ];
        
        find_all_base_positions(&map);
    }

    #[test]
    fn test_spawn_robots() {
        let base_positions = vec![(10, 10), (20, 20)];
        let robot_counts = vec![
            (RobotType::Explorer, 2),
            (RobotType::Miner, 1),
        ];
        
        let robots = spawn_robots_in_base(&base_positions, &robot_counts);
        
        assert_eq!(robots.len(), 3);
        assert_eq!(robots[0].robot_type, RobotType::Explorer);
        assert_eq!(robots[1].robot_type, RobotType::Explorer);
        assert_eq!(robots[2].robot_type, RobotType::Miner);
    }
}
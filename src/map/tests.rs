#[cfg(test)]
mod generator_tests {
    use super::super::generator::generate_map;
    use super::super::tile::Tile;

    #[test]
    fn test_generate_map_dimensions() {
        let map = generate_map(50, 30, 123);
        assert_eq!(map.len(), 30);
        assert_eq!(map[0].len(), 50);
    }

    #[test]
    fn test_generate_map_has_base() {
        let map = generate_map(40, 40, 456);
        let mut base_count = 0;
        
        for row in &map {
            for tile in row {
                if *tile == Tile::Base {
                    base_count += 1;
                }
            }
        }
        
        assert!(base_count > 0);
    }

    #[test]
    fn test_base_position_centered() {
        let map = generate_map(30, 20, 999);
        let base_size = usize::min(10, usize::min(30, 20) / 5);
        let expected_x = (30 - base_size) / 2;
        let expected_y = (20 - base_size) / 2;
        
        assert_eq!(map[expected_y][expected_x], Tile::Base);
    }
}

#[cfg(test)]
mod tile_tests {
    use super::super::tile::Tile;

    #[test]
    fn test_tile_to_char() {
        assert_eq!(Tile::Empty.to_char(), '.');
        assert_eq!(Tile::Obstacle.to_char(), '#');
        assert_eq!(Tile::Energy.to_char(), 'E');
        assert_eq!(Tile::Mineral.to_char(), 'M');
        assert_eq!(Tile::Science.to_char(), 'S');
        assert_eq!(Tile::Base.to_char(), 'B');
        assert_eq!(Tile::Robot.to_char(), 'R');
    }
}
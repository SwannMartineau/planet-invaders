#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Obstacle,
    Energy,
    Mineral,
    Science,
    Base,
    Robot,
}

impl Tile {
    pub fn to_char(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Obstacle => '#',
            Tile::Energy => '⚡',
            Tile::Mineral => '⛏',
            Tile::Science => '🔬',
            Tile::Base => '🏠',
            Tile::Robot => '🤖',
        }
    }

    pub fn is_consumable(&self) -> bool {
        matches!(self, Tile::Energy | Tile::Mineral)
    }
}
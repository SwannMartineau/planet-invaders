#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Obstacle,
    Energy,
    Mineral,
    Science,
}

impl Tile {
    pub fn to_char(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Obstacle => '#',
            Tile::Energy => '⚡',
            Tile::Mineral => '⛏',
            Tile::Science => '🔬',
        }
    }

    pub fn is_consumable(&self) -> bool {
        matches!(self, Tile::Energy | Tile::Mineral)
    }
}
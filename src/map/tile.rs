#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
            Tile::Empty => '.', //'⬜',
            Tile::Obstacle => '#', //'🪨',
            Tile::Energy => 'E', //'🔋',
            Tile::Mineral => 'M', //'💎',
            Tile::Science => 'S', //'🧬',
            Tile::Base => 'B', //'🏠',
            Tile::Robot => 'R', //'🤖',
        }
    }

    pub fn is_consumable(&self) -> bool {
        matches!(self, Tile::Energy | Tile::Mineral)
    }
}
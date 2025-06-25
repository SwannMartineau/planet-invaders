#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RobotType {
    Miner,
    EnergyCollector,
    Scientist,
    Explorer,
}

impl RobotType {
    pub fn to_char(&self) -> char {
        match self {
            RobotType::Miner => 'R', //'⛏️',
            RobotType::EnergyCollector => 'R', //'⚡',
            RobotType::Scientist => 'R', //'🔬',
            RobotType::Explorer => 'R', //'🔍',
        }
    }

    pub fn color(&self) -> ratatui::style::Color {
        match self {
            RobotType::Miner => ratatui::style::Color::Cyan,
            RobotType::EnergyCollector => ratatui::style::Color::Yellow,
            RobotType::Scientist => ratatui::style::Color::Magenta,
            RobotType::Explorer => ratatui::style::Color::Green,
        }
    }
}
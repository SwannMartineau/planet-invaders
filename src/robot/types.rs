#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RobotType {
    Miner,
    EnergyCollector,
    Scientist,
    Explorer,
}

impl RobotType {
    pub fn to_char(&self) -> char {
        match self {
            RobotType::Miner => '1',
            RobotType::EnergyCollector => '2',
            RobotType::Scientist => '3',
            RobotType::Explorer => '4',
        }
    }

    pub fn color(&self) -> ratatui::style::Color {
        match self {
            RobotType::Miner => ratatui::style::Color::Cyan,
            RobotType::EnergyCollector => ratatui::style::Color::Yellow,
            RobotType::Scientist => ratatui::style::Color::Magenta,
            RobotType::Explorer => ratatui::style::Color::Blue,
        }
    }
}
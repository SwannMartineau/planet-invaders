pub mod robot;
pub mod types;
#[cfg(test)]
mod tests;

pub use robot::{Robot, RobotState};
pub use types::RobotType;
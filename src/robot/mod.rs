pub mod robot;
pub mod types; // Si votre fichier s'appelle type.rs, changez cette ligne en: pub mod type;

pub use robot::{Robot, RobotState};
pub use types::RobotType; // Si votre fichier s'appelle type.rs, changez cette ligne en: pub use type::RobotType;
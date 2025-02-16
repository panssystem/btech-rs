use control::Player;
use units::Unit;

pub mod components;
pub mod constants;
mod control;
pub mod import;
pub mod movement;
pub mod random;
pub mod resources;
pub mod states;
pub mod systems;
mod units;

pub enum Commands {
    PlaceCommand,
    MoveCommand,
    FireCommand,
    TwistCommand,
    PhysicalCommand,
}

pub struct Side {
    name: String,
    command_queue: Vec<Commands>,
    units: Vec<Unit>,
    players: Vec<Player>,
}

impl Side {
    pub fn new(
        name: String,
        command_queue: Vec<Commands>,
        units: Vec<Unit>,
        players: Vec<Player>,
    ) -> Self {
        Self {
            name,
            command_queue,
            units,
            players,
        }
    }
}

pub mod game {
    use super::Side;

    pub struct Game {
        pub(crate) name: String,
        pub(crate) description: String,
        pub(crate) sides: Vec<Side>,
    }

    impl Game {
        /// Creates a new [`Game`].
        /// ```
        /// use btech_rs::{game::Game, Side};
        /// let s = Side::new("name".to_string(), vec![], vec![], vec![]);
        /// let g = Game::new("Test Game".to_string(), "Just a test".to_string(), vec![s]);
        /// if let None = g.get_side(0) { assert!(false) }
        /// ```
        pub fn new(name: String, description: String, sides: Vec<Side>) -> Self {
            Self {
                name,
                description,
                sides,
            }
        }
        pub fn get_side_count(&self) -> usize {
            self.sides.len()
        }
        pub fn get_side(&self, n: usize) -> Option<&Side> {
            self.sides.get(n)
        }
        pub fn add_side(&mut self, name: String) -> Option<&Side> {
            let new_side = Side::new(name,vec![],vec![],vec![]);
            self.sides.push(new_side);
            self.sides.last()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game() {
        let s = Side::new("name".to_string(), vec![], vec![], vec![]);
        let g = game::Game::new("Test Game".to_string(), "Just a test".to_string(), vec![s]);
        assert_eq!(g.sides.len(), 1)
    }
}

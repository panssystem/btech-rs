use control::Player;
use units::Unit;

mod control;
mod units;

pub mod import {
    use std::collections::{hash_map, HashMap};

    #[derive(Debug)]
    pub struct FileImport {
        entries: HashMap<String, FileEntry>,
    }

    impl FileImport {
        pub fn new() -> Self {
            Self {
                entries: hash_map::HashMap::new(),
            }
        }
        pub fn add_entry(&mut self, k: String, v: FileEntry) {
            self.entries.insert(k, v);
        }
    }

    #[derive(Debug)]
    pub enum FileEntry {
        Single(String),
        Block(Block),
        Empty,
    }
    #[derive(Debug)]
    pub enum BlockEntry {
        Str(String),
    }
    #[derive(Debug)]
    pub struct Block {
        entries: Vec<BlockEntry>,
    }

    impl Block {
        pub fn new(entries: Vec<BlockEntry>) -> Self {
            Self { entries }
        }
    }
}
pub enum TurnPhase {
    InitiativePhase,
    MovementPhase(MovementSubphase),
    WeaponAttackPhase,
    PhysicalAttackPhase,
    HeatPhase,
    EndPhase,
}

pub enum MovementSubphase {
    GroundMovementPhase,
    AerospaceMovementPhase,
}
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

mod game {
    use super::Side;

    pub struct Game {
        pub(crate) name: String,
        pub(crate) description: String,
        pub(crate) sides: Vec<Side>,
    }

    impl Game {
        /// Creates a new [`Game`].
        /// ```
        /// use btech_rs::{Game, Side};
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

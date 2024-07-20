use crate::movement::{MoveAction, MoveMode};

#[derive(Clone, Debug, PartialEq)]
pub struct MoveCommand {
    unit_id: String,
    mode: MoveMode,
    steps: Vec<MoveAction>,
}

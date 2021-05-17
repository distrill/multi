use serde::{Serialize, Deserialize};

use crate::world::WorldState;

#[derive(Debug, Serialize, Deserialize)]
pub enum Cmd {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FromClientMessage {
    pub cmds: Vec<Cmd>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FromServerMessage {
    world: WorldState,
}

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
pub enum FromClientMessage {
    Update {
        cmds: Vec<Cmd>
    },
    Init {
        username: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FromServerMessage {
    Tick {
        world: WorldState,
    },
    ConnectionSuccess,
    ConnectionError(String),
}

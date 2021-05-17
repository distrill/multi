use nanoid::nanoid;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: String,
    pub pos: Pos,
}

impl Player {
    pub fn new() -> Player {
        Player {
            id: nanoid!(),
            pos: Pos { x: 0, y: 0 },
        }
    }
}

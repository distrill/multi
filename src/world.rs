use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::map::Map;
use crate::player::Player;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorldState {
    pub players: HashMap<String, Player>,
    pub map: Map,
}

impl WorldState {
    pub fn new() -> WorldState {
        let players = HashMap::new();
        let map = Map::new();

        WorldState { players, map }
    }
}

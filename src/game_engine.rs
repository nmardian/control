use crate::fighter::Fighter;
use std::collections::HashMap;

pub struct GameEngine {
    pub all_movers: HashMap<u32, Fighter>,
}

impl GameEngine {
    pub const MIN_X_COORD: u32 = 0;
    pub const MAX_X_COORD: u32 = 375500;
    pub const MIN_Y_COORD: u32 = 0;
    pub const MAX_Y_COORD: u32 = 375500;

    pub const HEADING_FULL_CIRCLE: u32 = 360;
    pub const HEADING_HALF_CIRCLE: u32 = 180;

    pub fn new() -> GameEngine {
        let mut movers: HashMap<u32, Fighter> = HashMap::new();

        GameEngine { all_movers: movers }
    }
}

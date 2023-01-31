use crate::fighter::Fighter;
use std::collections::HashMap;

pub struct GameEngine {
    pub all_fighters: HashMap<String, Fighter>,
}

impl GameEngine {
    pub const MIN_X_COORD: u32 = 0;
    pub const MAX_X_COORD: u32 = 375500;
    pub const MIN_Y_COORD: u32 = 0;
    pub const MAX_Y_COORD: u32 = 375500;

    pub const HEADING_FULL_CIRCLE: u32 = 360;
    pub const HEADING_HALF_CIRCLE: u32 = 180;

    pub fn new() -> GameEngine {
        let mut fighters: HashMap<String, Fighter> = HashMap::new();

        GameEngine { all_fighters: fighters }
    }

    pub fn add_fighter(&mut self, fighter: Fighter) -> bool {
        let result:bool = !self.all_fighters.contains_key(&fighter.id);

        if result {
            self.all_fighters.insert(fighter.id.clone(), fighter);
        }

        result
    }
}

#[test]
fn add_fighter_test() {
    let mut sut_game_engine: GameEngine = GameEngine::new();

    let fighter_one: Fighter = Fighter::new("Alpha".to_string());
    let fighter_one_dupe: Fighter = Fighter::new("Alpha".to_string());
    let fighter_two: Fighter = Fighter::new("Bravo".to_string());

    assert_eq!(true, sut_game_engine.add_fighter(fighter_one));
    assert_eq!(false, sut_game_engine.add_fighter(fighter_one_dupe));
    assert_eq!(true, sut_game_engine.add_fighter(fighter_two));
}

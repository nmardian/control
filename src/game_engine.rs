use crate::fighter::Fighter;
use std::collections::HashMap;

#[derive (serde::Serialize)]
pub struct GameEngine {
    pub all_fighters: HashMap<String, Fighter>,
    ended: bool,
}

impl GameEngine {
    pub const MIN_X_COORD: u32 = 0;
    pub const MAX_X_COORD: u32 = 375500;
    pub const MIN_Y_COORD: u32 = 0;
    pub const MAX_Y_COORD: u32 = 375500;

    pub const HEADING_FULL_CIRCLE: u32 = 360;
    pub const HEADING_HALF_CIRCLE: u32 = 180;

    pub fn new() -> GameEngine {
        let fighters: HashMap<String, Fighter> = HashMap::new();

        GameEngine {
            all_fighters: fighters,
            ended: false,
        }
    }

    pub fn add_fighter(&mut self, fighter: Fighter) -> bool {
        let result: bool = !self.all_fighters.contains_key(&fighter.id);

        if result {
            self.all_fighters.insert(fighter.id.clone(), fighter);
        }

        result
    }

    pub fn get_fighter(&self, id: String) -> Option<&Fighter> {
        self.all_fighters.get(&id)
    }

    pub fn tick(&mut self) {
        for cur_fighter in self.all_fighters.values_mut() {
            cur_fighter.move_fighter();
        }
    }

    pub fn get_game_state(&self) -> String {
        let game_state_json: String = serde_json::to_string_pretty(&self.all_fighters).unwrap();

        game_state_json
    }

    pub fn is_ended(&self) -> bool
    {
        self.ended
    }
}

#[test]
fn add_get_fighter_test() {
    let mut sut_game_engine: GameEngine = GameEngine::new();

    let id_alpha: &str = "Alpha";
    let id_bravo: &str = "Bravo";

    let fighter_one: Fighter = Fighter::new(id_alpha.to_string());
    let fighter_one_dupe: Fighter = Fighter::new(id_alpha.to_string());
    let fighter_two: Fighter = Fighter::new(id_bravo.to_string());

    assert_eq!(true, sut_game_engine.add_fighter(fighter_one));
    assert_eq!(false, sut_game_engine.add_fighter(fighter_one_dupe));
    assert_eq!(true, sut_game_engine.add_fighter(fighter_two));

    assert!(sut_game_engine.get_fighter(id_alpha.to_string()).is_some());
    assert!(sut_game_engine.get_fighter(id_bravo.to_string()).is_some());
    assert!(sut_game_engine.get_fighter("Charlie".to_string()).is_none());
}

#[test]
fn tick_test() {
    let mut sut_game_engine: GameEngine = GameEngine::new();

    let id: &str = "Alpha";
    let heading_north_east: u32 = 45;
    let initial_x_coord: u32 = GameEngine::MAX_X_COORD / 2;
    let initial_y_coord: u32 = GameEngine::MAX_Y_COORD / 2;
    let speed: u32 = 10;

    let mut fighter: Fighter = Fighter::new(id.to_string());

    fighter.set_inertial_data(heading_north_east, speed, initial_x_coord, initial_y_coord);

    sut_game_engine.add_fighter(fighter);

    sut_game_engine.tick();

    if let Some(returned_fighter) = sut_game_engine.get_fighter(id.to_string()) {
        assert!(returned_fighter.x_coord != initial_x_coord);
        assert!(returned_fighter.y_coord != initial_y_coord);
    }
}

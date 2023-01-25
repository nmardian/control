use crate::game_engine::GameEngine;

pub struct Fighter {
    pub id: String,
    pub heading: u32,
    pub cur_speed: u32,
    pub x_coord: u32,
    pub y_coord: u32,
}

impl Fighter {
    const MIN_HEADING:u32 = 0;
    const MAX_HEADING: u32 = 259;
    const MIN_SPEED:u32 = 0;
    const MAX_SPEED: u32 = 838;
    const TURN_RATE: u32 = 5;

    pub fn new(id: String) -> Fighter {
        Fighter {
            id: id,
            heading: 0,
            cur_speed: 0,
            x_coord: 0,
            y_coord: 0,
        }
    }

    pub fn set_inertial_data(&self, heading: u32, cur_speed: u32, x_coord: u32, y_coord: u32) -> bool {
        let mut result: bool = false;

        if heading <= Fighter::MAX_HEADING
            && cur_speed <= Fighter::MAX_SPEED
            && x_coord <= GameEngine::MAX_X_COORD
            && y_coord <= GameEngine::MAX_Y_COORD
        {
            result = true;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_inertial_data_tests() {
        
        let sut_fighter: crate::fighter::Fighter = Fighter::new("Alpha".to_string());

        // minimums
        assert_eq!(true, sut_fighter.set_inertial_data(Fighter::MIN_HEADING, Fighter::MIN_SPEED, GameEngine::MIN_X_COORD, GameEngine::MIN_Y_COORD));

        // maximums
        assert_eq!(true, sut_fighter.set_inertial_data(Fighter::MAX_HEADING, Fighter::MAX_SPEED, GameEngine::MAX_X_COORD, GameEngine::MAX_Y_COORD));

        // high heading
        assert_eq!(false, sut_fighter.set_inertial_data(Fighter::MAX_HEADING + 1, Fighter::MAX_SPEED, GameEngine::MAX_X_COORD, GameEngine::MAX_Y_COORD));

        // high speed
        assert_eq!(false, sut_fighter.set_inertial_data(Fighter::MAX_HEADING, Fighter::MAX_SPEED + 1, GameEngine::MAX_X_COORD, GameEngine::MAX_Y_COORD));

        // high x coord
        assert_eq!(false, sut_fighter.set_inertial_data(Fighter::MAX_HEADING, Fighter::MAX_SPEED, GameEngine::MAX_X_COORD + 1, GameEngine::MAX_Y_COORD));

        // high y coord
        assert_eq!(false, sut_fighter.set_inertial_data(Fighter::MAX_HEADING, Fighter::MAX_SPEED, GameEngine::MIN_X_COORD, GameEngine::MAX_Y_COORD + 1));
    }

}

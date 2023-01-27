use std::cmp;

use crate::game_engine::GameEngine;

pub struct Fighter {
    pub id: String,
    pub cur_heading_degrees: u32,
    pub desired_heading_degrees: u32,
    pub cur_speed: u32,
    pub x_coord: u32,
    pub y_coord: u32,
}

impl Fighter {
    const MIN_HEADING: u32 = 0;
    const MAX_HEADING: u32 = 359;
    const MIN_SPEED: u32 = 0;
    const MAX_SPEED: u32 = 838;
    const TURN_RATE: u32 = 10;
    const ACCL_DECL: u32 = 5;

    pub fn new(id: String) -> Fighter {
        Fighter {
            id: id,
            cur_heading_degrees: 0,
            desired_heading_degrees: 0,
            cur_speed: 0,
            x_coord: 0,
            y_coord: 0,
        }
    }

    pub fn set_inertial_data(
        &mut self,
        heading: u32,
        speed: u32,
        x_coord: u32,
        y_coord: u32,
    ) -> bool {
        let mut result: bool = false;

        if heading <= Fighter::MAX_HEADING
            && speed <= Fighter::MAX_SPEED
            && x_coord <= GameEngine::MAX_X_COORD
            && y_coord <= GameEngine::MAX_Y_COORD
        {
            self.cur_heading_degrees = heading;
            self.cur_speed = speed;
            self.x_coord = x_coord;
            self.y_coord = y_coord;

            result = true;
        }

        result
    }

    pub fn set_new_heading(&mut self, new_heading_degrees: u32) -> bool {
        let mut result = false;

        if new_heading_degrees <= Fighter::MAX_HEADING {
            self.desired_heading_degrees = new_heading_degrees;
            result = true;
        }

        result
    }

    pub fn move_fighter(&mut self) {
        if self.desired_heading_degrees != self.cur_heading_degrees {

            let degrees_to_desired_heading_left:u32 = (GameEngine::HEADING_FULL_CIRCLE + self.cur_heading_degrees
            - self.desired_heading_degrees) % GameEngine::HEADING_FULL_CIRCLE;

            let degrees_to_desired_heading_right:u32 = (GameEngine::HEADING_FULL_CIRCLE + self.cur_heading_degrees
            + self.desired_heading_degrees) % GameEngine::HEADING_FULL_CIRCLE;

            // turn right
            if degrees_to_desired_heading_left > GameEngine::HEADING_HALF_CIRCLE
            {
                if degrees_to_desired_heading_right <= Fighter::TURN_RATE {
                    self.cur_heading_degrees = self.desired_heading_degrees;
                }
                else
                {
                    self.cur_heading_degrees = (GameEngine::HEADING_FULL_CIRCLE
                        + self.cur_heading_degrees
                        + Fighter::TURN_RATE)
                        % GameEngine::HEADING_FULL_CIRCLE;
                }
            }
            // turn left
            else {
                if degrees_to_desired_heading_left <= Fighter::TURN_RATE {
                    self.cur_heading_degrees = self.desired_heading_degrees;
                }
                else {
                    self.cur_heading_degrees = (GameEngine::HEADING_FULL_CIRCLE + self.cur_heading_degrees
                        - Fighter::TURN_RATE)
                        % GameEngine::HEADING_FULL_CIRCLE;
                }
            }

            // turn will decrease speed
            self.cur_speed = cmp::max(Fighter::MIN_SPEED, self.cur_speed - Fighter::ACCL_DECL);
        } else {
            // flying straight increases speed
            self.cur_speed = cmp::min(Fighter::MAX_SPEED, self.cur_speed + Fighter::ACCL_DECL);
        }

        // TODO: Change x_coord and y_coord of the Fighter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_inertial_data_tests() {
        let mut sut_fighter: Fighter = Fighter::new("Alpha".to_string());

        // minimums
        assert_eq!(
            true,
            sut_fighter.set_inertial_data(
                Fighter::MIN_HEADING,
                Fighter::MIN_SPEED,
                GameEngine::MIN_X_COORD,
                GameEngine::MIN_Y_COORD
            )
        );

        // maximums
        assert_eq!(
            true,
            sut_fighter.set_inertial_data(
                Fighter::MAX_HEADING,
                Fighter::MAX_SPEED,
                GameEngine::MAX_X_COORD,
                GameEngine::MAX_Y_COORD
            )
        );

        // high heading
        assert_eq!(
            false,
            sut_fighter.set_inertial_data(
                Fighter::MAX_HEADING + 1,
                Fighter::MAX_SPEED,
                GameEngine::MAX_X_COORD,
                GameEngine::MAX_Y_COORD
            )
        );

        // high speed
        assert_eq!(
            false,
            sut_fighter.set_inertial_data(
                Fighter::MAX_HEADING,
                Fighter::MAX_SPEED + 1,
                GameEngine::MAX_X_COORD,
                GameEngine::MAX_Y_COORD
            )
        );

        // high x coord
        assert_eq!(
            false,
            sut_fighter.set_inertial_data(
                Fighter::MAX_HEADING,
                Fighter::MAX_SPEED,
                GameEngine::MAX_X_COORD + 1,
                GameEngine::MAX_Y_COORD
            )
        );

        // high y coord
        assert_eq!(
            false,
            sut_fighter.set_inertial_data(
                Fighter::MAX_HEADING,
                Fighter::MAX_SPEED,
                GameEngine::MIN_X_COORD,
                GameEngine::MAX_Y_COORD + 1
            )
        );
    }

    #[test]
    fn move_fighter_test_turn_right() {
        let mut sut_fighter: Fighter = Fighter::new("Alpha".to_string());

        let mut initial_heading: u32 = 0;
        let mut desired_heading: u32 = 90;
        let initial_speed: u32 = 100;
        let x_coord: u32 = 0;
        let y_coord: u32 = 0;

        sut_fighter.set_inertial_data(initial_heading, initial_speed, x_coord, y_coord);
        assert_eq!(true, sut_fighter.set_new_heading(desired_heading));

        for move_count in 1..3 {
            sut_fighter.move_fighter();

            assert_eq!(
                initial_heading + (move_count * Fighter::TURN_RATE),
                sut_fighter.cur_heading_degrees
            );

            assert_eq!(initial_speed - (move_count * Fighter::ACCL_DECL), sut_fighter.cur_speed);
        }

        initial_heading = 180;
        desired_heading = 270;
        sut_fighter.set_inertial_data(initial_heading, initial_speed, x_coord, y_coord);
        assert_eq!(true, sut_fighter.set_new_heading(desired_heading));

        for move_count in 1..3 {
            sut_fighter.move_fighter();

            assert_eq!(
                initial_heading + (move_count * Fighter::TURN_RATE),
                sut_fighter.cur_heading_degrees
            );

            assert_eq!(initial_speed - (move_count * Fighter::ACCL_DECL), sut_fighter.cur_speed);
        }
    }

    #[test]
    fn move_fighter_test_turn_left() {
        let mut sut_fighter: Fighter = Fighter::new("Alpha".to_string());

        let mut initial_heading: u32 = 180;
        let mut desired_heading: u32 = 90;
        let initial_speed: u32 = 100;
        let x_coord: u32 = 0;
        let y_coord: u32 = 0;

        sut_fighter.set_inertial_data(initial_heading, initial_speed, x_coord, y_coord);
        assert_eq!(true, sut_fighter.set_new_heading(desired_heading));

        for move_count in 1..3 {
            sut_fighter.move_fighter();

            assert_eq!(
                initial_heading - (move_count * Fighter::TURN_RATE),
                sut_fighter.cur_heading_degrees
            );

            assert_eq!(initial_speed - (move_count * Fighter::ACCL_DECL), sut_fighter.cur_speed);
        }

        initial_heading = 0;
        desired_heading = 270;

        sut_fighter.set_inertial_data(initial_heading, initial_speed, x_coord, y_coord);
        assert_eq!(true, sut_fighter.set_new_heading(desired_heading));

        for move_count in 1..3 {
            sut_fighter.move_fighter();

            assert_eq!(
                360 - (move_count * Fighter::TURN_RATE),
                sut_fighter.cur_heading_degrees
            );

            assert_eq!(initial_speed - (move_count * Fighter::ACCL_DECL), sut_fighter.cur_speed);
        }
    }

    #[test]
    fn move_fighter_test_no_overturn() {
        let mut sut_fighter: Fighter = Fighter::new("Alpha".to_string());

        let initial_heading: u32 = 0;
        let mut desired_heading: u32 = 5;
        let cur_speed: u32 = 100;
        let x_coord: u32 = 0;
        let y_coord: u32 = 0;

        sut_fighter.set_inertial_data(initial_heading, cur_speed, x_coord, y_coord);
        assert_eq!(true, sut_fighter.set_new_heading(desired_heading));

        for _move_count in 1..3 {
            sut_fighter.move_fighter();

            assert_eq!(
                desired_heading,
                sut_fighter.cur_heading_degrees
            );    
        }

        desired_heading = 355;
        sut_fighter.set_inertial_data(initial_heading, cur_speed, x_coord, y_coord);
        assert_eq!(true, sut_fighter.set_new_heading(desired_heading));

        for _move_count in 1..3 {
            sut_fighter.move_fighter();

            assert_eq!(
                desired_heading,
                sut_fighter.cur_heading_degrees
            );    
        }
    }
}

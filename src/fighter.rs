use std::cmp;

use crate::game_engine::GameEngine;
use crate::math_util;

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
            self.desired_heading_degrees = heading;
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
            let degrees_to_desired_heading_left: u32 = (GameEngine::HEADING_FULL_CIRCLE
                + self.cur_heading_degrees
                - self.desired_heading_degrees)
                % GameEngine::HEADING_FULL_CIRCLE;

            let degrees_to_desired_heading_right: u32 = (GameEngine::HEADING_FULL_CIRCLE
                + self.cur_heading_degrees
                + self.desired_heading_degrees)
                % GameEngine::HEADING_FULL_CIRCLE;

            // turn right
            if degrees_to_desired_heading_left > GameEngine::HEADING_HALF_CIRCLE {
                if degrees_to_desired_heading_right <= Fighter::TURN_RATE {
                    self.cur_heading_degrees = self.desired_heading_degrees;
                } else {
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
                } else {
                    self.cur_heading_degrees = (GameEngine::HEADING_FULL_CIRCLE
                        + self.cur_heading_degrees
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

        let delta_x: i32 =
            math_util::get_x_component_of_speed(self.cur_speed, self.cur_heading_degrees);
        let delta_y: i32 =
            math_util::get_y_component_of_speed(self.cur_speed, self.cur_heading_degrees);

        if delta_x.is_positive() {
            let temp_x: u32 = self.x_coord + delta_x as u32;
            self.x_coord = cmp::min(GameEngine::MAX_X_COORD, temp_x);
        } else {
            let temp_x: i32 = self.x_coord as i32 + delta_x;
            self.x_coord = cmp::max(GameEngine::MIN_X_COORD as i32, temp_x) as u32;
        }

        if delta_y.is_positive() {
            let temp_y: u32 = self.y_coord + delta_y as u32;
            self.y_coord = cmp::min(GameEngine::MAX_Y_COORD, temp_y);
        } else {
            let temp_y: i32 = self.y_coord as i32 + delta_y;
            self.y_coord = cmp::max(GameEngine::MIN_Y_COORD as i32, temp_y) as u32;
        }
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

            assert_eq!(
                initial_speed - (move_count * Fighter::ACCL_DECL),
                sut_fighter.cur_speed
            );
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

            assert_eq!(
                initial_speed - (move_count * Fighter::ACCL_DECL),
                sut_fighter.cur_speed
            );
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

            assert_eq!(
                initial_speed - (move_count * Fighter::ACCL_DECL),
                sut_fighter.cur_speed
            );
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

            assert_eq!(
                initial_speed - (move_count * Fighter::ACCL_DECL),
                sut_fighter.cur_speed
            );
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

            assert_eq!(desired_heading, sut_fighter.cur_heading_degrees);
        }

        desired_heading = 355;
        sut_fighter.set_inertial_data(initial_heading, cur_speed, x_coord, y_coord);
        assert_eq!(true, sut_fighter.set_new_heading(desired_heading));

        for _move_count in 1..3 {
            sut_fighter.move_fighter();

            assert_eq!(desired_heading, sut_fighter.cur_heading_degrees);
        }
    }

    // test move_fighter in straight line to max speed

    #[test]
    fn move_fighter_max_x() {
        let mut sut_fighter: Fighter = Fighter::new("Alpha".to_string());

        let heading_east: u32 = 90;
        let speed: u32 = 10;
        let initial_x: u32 = GameEngine::MAX_X_COORD - 5;
        let initial_y = GameEngine::MAX_Y_COORD / 2;

        sut_fighter.set_inertial_data(heading_east, speed, initial_x, initial_y);

        for _move_count in 1..3 {
            sut_fighter.move_fighter();
            assert_eq!(GameEngine::MAX_X_COORD, sut_fighter.x_coord);
        }
    }

    #[test]
    fn move_fighter_min_x() {
        let mut sut_fighter: Fighter = Fighter::new("Alpha".to_string());

        let heading_west: u32 = 270;
        let speed: u32 = 10;
        let initial_x: u32 = GameEngine::MIN_X_COORD + 5;
        let initial_y = GameEngine::MAX_Y_COORD / 2;

        sut_fighter.set_inertial_data(heading_west, speed, initial_x, initial_y);

        for _move_count in 1..3 {
            sut_fighter.move_fighter();
            assert_eq!(GameEngine::MIN_X_COORD, sut_fighter.x_coord);
        }
    }

    #[test]
    fn move_fighter_max_y() {
        let mut sut_fighter: Fighter = Fighter::new("Alpha".to_string());

        let heading_north: u32 = 0;
        let speed: u32 = 10;
        let initial_x: u32 = GameEngine::MAX_X_COORD / 2;
        let initial_y = GameEngine::MAX_Y_COORD - 5;

        sut_fighter.set_inertial_data(heading_north, speed, initial_x, initial_y);

        for _move_count in 1..3 {
            sut_fighter.move_fighter();
            assert_eq!(GameEngine::MAX_Y_COORD, sut_fighter.y_coord);
        }
    }

    #[test]
    fn move_fighter_min_y() {
        let mut sut_fighter: Fighter = Fighter::new("Alpha".to_string());

        let heading_south: u32 = 180;
        let speed: u32 = 10;
        let initial_x: u32 = GameEngine::MAX_X_COORD / 2;
        let initial_y = GameEngine::MIN_Y_COORD + 5;

        sut_fighter.set_inertial_data(heading_south, speed, initial_x, initial_y);

        for _move_count in 1..3 {
            sut_fighter.move_fighter();
            assert_eq!(GameEngine::MIN_Y_COORD, sut_fighter.y_coord);
        }
    }

    #[test]
    fn move_fighter_accl() {
        let mut sut_fighter: Fighter = Fighter::new("Alpha".to_string());

        let heading_east: u32 = 90;
        let initial_speed: u32 = 100;
        let initial_x: u32 = GameEngine::MAX_X_COORD / 2;
        let initial_y: u32 = GameEngine::MAX_Y_COORD / 2;

        sut_fighter.set_inertial_data(heading_east, initial_speed, initial_x, initial_y);

        for move_count in 1..3 {
            sut_fighter.move_fighter();
            assert_eq!(
                initial_speed + (move_count * Fighter::ACCL_DECL),
                sut_fighter.cur_speed
            );
        }
    }
}

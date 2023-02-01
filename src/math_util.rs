const PI_RADIANS:f64 = 180.0 / std::f64::consts::PI;

fn get_x_component_of_speed(speed: u32, angle: u32) -> i32 {
    (speed as f64 * (angle as f64 / PI_RADIANS).sin()).round() as i32
}

fn get_y_component_of_speed(speed: u32, angle: u32) -> i32 {
    (speed as f64 * (angle as f64 / PI_RADIANS).cos()).round() as i32
}

#[test]
fn get_x_component_of_speed_test() {

    // heading 0, no change
    assert_eq!(0, get_x_component_of_speed(10, 0));

    // heading 180, no change
    assert_eq!(0, get_x_component_of_speed(10, 180));

    // heading 90, max change
    assert_eq!(10, get_x_component_of_speed(10, 90));

    // heading 270, max change
    assert_eq!(-10, get_x_component_of_speed(10, 270));
}

#[test]
fn get_y_component_of_speed_test() {

    // heading 0, max change
    assert_eq!(10, get_y_component_of_speed(10, 0));

    // heading 180, max change
    assert_eq!(-10, get_y_component_of_speed(10, 180));

    // heading 90, no change
    assert_eq!(0, get_y_component_of_speed(10, 90));

    // heading 270, no change
    assert_eq!(0, get_y_component_of_speed(10, 270));
}
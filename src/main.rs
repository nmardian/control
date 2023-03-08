use crate::game_engine::GameEngine;
use crate::fighter::Fighter;

use std::thread;
use std::time::Duration;

pub mod fighter;
pub mod game_engine;

pub mod math_util;
fn main() {
    let mut game_engine: GameEngine = GameEngine::new();

    let mut fighter_alpha: Fighter = Fighter::new("Alpha".to_string());
    game_engine.add_fighter(fighter_alpha);

    let nats_connection = nats::connect("nats://127.0.0.1:4222").unwrap();

    let game_thread = thread::spawn(move || {

        while !game_engine.is_ended()
        {
            game_engine.tick();

            let game_state:String = game_engine.get_game_state();
            nats_connection.publish("game-state", game_state).unwrap();

            thread::sleep(Duration::from_secs(1));
        }
    });

    while true
    {
        thread::sleep(Duration::from_secs(1));
    }
}
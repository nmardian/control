use crate::fighter::Fighter;
use crate::game_engine::GameEngine;

use std::string;
use std::string::FromUtf8Error;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

pub mod fighter;
pub mod game_engine;

pub mod math_util;
fn main() {
    let mut game_engine: GameEngine = GameEngine::new();

    let mut fighter_alpha: Fighter = Fighter::new("Alpha".to_string());
    game_engine.add_fighter(fighter_alpha);

    let (gamestate_tx, gamestate_rx) = mpsc::channel();
    let (command_tx, command_rx) = mpsc::channel();

    let nats_connection =
        nats::connect("nats://127.0.0.1:4222").expect("Error connecting to nats server");

    let nats_sub_commands = nats_connection
        .subscribe("commands")
        .expect("Error subscribing to commands topic");

    let command_handler_thread = thread::spawn(move || {
        for msg in nats_sub_commands.messages() {
            command_tx
                .send(msg)
                .expect("Error sending command to game_engine");
        }
    });

    let game_thread = thread::spawn(move || {
        while !game_engine.is_ended() {
            let msg_result = command_rx.try_recv();

            match msg_result {
                Ok(msg) => {
                    let readable_msg =
                        String::from_utf8(msg.data).expect("Error converting incoming message");

                    println!("game_thread got message: {:?}", readable_msg)
                }
                Err(TryRecvError::Disconnected) => {
                    panic!("Error in game thread, command channel disconnected")
                }
                Err(TryRecvError::Empty) => (),
            };

            game_engine.tick();

            gamestate_tx.send(game_engine.get_game_state()).unwrap();

            thread::sleep(Duration::from_secs(1));
        }
    });

    for cur_gamestate in gamestate_rx {
        nats_connection
            .publish("game-state", cur_gamestate)
            .unwrap();
    }

    command_handler_thread.join().unwrap();
    game_thread.join().unwrap();
}

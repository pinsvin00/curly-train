mod ball;
use ball::Ball;

mod drawable;
use drawable::Drawable;

mod globals;
use globals::X_SIZE;
use globals::Y_SIZE;

mod vector;
use vector::Vector2i;
mod rect;
use rect::Rectangle;

mod grid;
use grid::Grid;

mod connectioninfo;
mod gameinfo;

mod connection;
use connection::{Connection};
use std::sync::mpsc;


// for trait
mod game;
use game::Game;

mod utils;
use utils::is_position_valid;
use utils::get;
use std::sync::Arc;

extern crate termion;

use std::io;

// TODO:
// Easy mode -> Leaving predicted trail where ball will land
// Online move
// Sounds -> On hit
// Graphics -> Colors
// Simple AI 


fn convert_input_yn(info : &str) -> bool {
    println!("{}",info);
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    choice = choice.to_ascii_uppercase();
    println!("{}", choice);
    if choice == String::from("Y\n") { 
        return true;
    } else if choice == String::from("N\n"){
        return false;
    }

    return false;
}



fn main() {

    println!("SUPER MEGA GIGA ULTRA PETA PONG SMGUPP!");
    let host = convert_input_yn("HOST? Y/N");


    let mut game = Game::new(X_SIZE, Y_SIZE);

    let (tx, rx) = mpsc::channel();

    let tx_clone = tx.clone();

    println!("{}", host);

    let handle = std::thread::spawn(move || {
        let mut conn : Connection;
        if host  {
            println!("sus");
            conn = Connection::listener();
        }
        else {
            let addr = String::from("127.0.0.1:7999");
            conn = Connection::connector(addr).unwrap();
            conn.acquire();
        }

        conn.acquire();
        conn.handshake();

        loop {
            conn.game_tick();
            let copy = conn.curr_game_info.clone();
            tx_clone.send(copy).unwrap();
        } 
        
    });

    let res = handle.join();
    


    // let mut solo = convert_input_yn("Do you have a friend to play with? Y/N");


    // while !game.is_ended() { 

    //     game.loop_logic();
    // }
}

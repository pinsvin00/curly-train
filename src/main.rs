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
use gameinfo::GameInfo;

mod connection;
use connection::{Connection};
use std::sync::mpsc;


// for trait
mod game;
use game::Game;

mod utils;
use utils::*;
use std::sync::Arc;

extern crate termion;

use std::io;

// TODO:
// Better interface for online gaming!
// Make this code look cleaner
// Online move
// Sounds -> On hit
// Graphics -> Colors
// Simple AI 


fn convert_input_yn(info : &str) -> bool {
    println!("{}", info);
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    choice = choice.to_ascii_uppercase();
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

    let (tx, rx) = mpsc::channel();
    let (tx_, rx_) = mpsc::channel();

    std::thread::spawn(move || {
        let mut conn: Connection;
        if host {
            conn = Connection::listener(tx, rx_);    
        }
        else { 
            conn = Connection::connector(String::from("0.0.0.0:7999"), tx, rx_).unwrap();
        }

        conn.acquire();
    });

    let mut game = Game::new(X_SIZE, Y_SIZE, (host as u8) + 1, tx_, rx);
    while !game.is_ended() { 
        game.loop_logic();
    }
    switch_to_normal();
}

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

mod gamestate;
use gamestate::GCSignalType;
use gamestate::GameState;


mod gameinfo;


mod connection;
use connection::{Connection};
use std::sync::mpsc;


// for trait
mod game;
use game::Game;

mod utils;
use utils::*;

extern crate termion;

use std::io;

// Known Issues
/*
    When ball has high speed collisions may not work properly.
    Often desyncs
*/


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
    let mut ip_addr = String::new();

    if !host {
        io::stdin().read_line(&mut ip_addr).expect("Failed to read the line");   
    }


    let (tx, rx) = mpsc::channel();
    let (tx_, rx_) = mpsc::channel();
    let (conn_sender, conn_recv) = mpsc::channel();


    std::thread::spawn(move || {
        let mut conn: Connection;
        if host {
            conn = Connection::listener(tx, rx_);    
        }
        else { 
            conn = Connection::connector(ip_addr, tx, rx_).unwrap();
        }
        conn.acquire(conn_sender);
    });

    let mut game_info = conn_recv.recv().unwrap();
    game_info.is_host = host;
    let mut game = Game::new(game_info, (host as u8) + 1, tx_, rx);
    while !game.is_ended() { 
        game.loop_logic();
    }
    switch_to_normal();
}

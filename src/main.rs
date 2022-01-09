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

use std::net::{TcpListener, TcpStream};
// for trait

mod game;
use game::Game;

mod utils;
use utils::is_position_valid;
use utils::get;


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

    if choice == "Y" { 
        return true;
    } else if choice == "N"{
        return false;
    }

    return false;
}



fn main() {

    println!("SUPER MEGA GIGA ULTRA PETA PONG SMGUPP!");


    let listener = TcpListener::bind("127.0.0.1:80");
    // let mut easy = convert_input_yn("Do you wish to play in easy mode? Y/N");
    // let mut solo = convert_input_yn("Do you have a friend to play with? Y/N");

    let mut game = Game::new(X_SIZE, Y_SIZE);
    while !game.is_ended() { 
        game.loop_logic();
    }
}

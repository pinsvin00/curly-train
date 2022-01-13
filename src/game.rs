

use crate::Rectangle;
use crate::Grid;
use crate::Ball;
use crate::Vector2i;
use crate::Connection;
use crate::GameInfo;
use std::sync::mpsc::{Receiver, Sender};
use crate::Drawable;
use std::sync::mpsc;
use termion::raw::IntoRawMode;
use std::io::{stdout};
use termion::input::TermRead;

use std::thread;



pub struct Game {
    paddle1: Rectangle,
    paddle2: Rectangle,
    ball: Ball,

    grid: Grid,

    frame_interval: std::time::Duration,
    score_info_interval: std::time::Duration,

    ended: bool,
    score: Vec<u16>,

    stdin : Option<termion::input::Keys<termion::AsyncReader>>,
    stdout: Option<termion::raw::RawTerminal<std::io::Stdout>>,

    rx : Receiver<GameInfo>,
    tx : Sender<GameInfo>,
}

impl Game {
    pub fn new(x_size: usize, y_size: usize, host: bool, rx: Receiver<GameInfo>, tx : Sender<GameInfo>) -> Self { 
        let paddle_speed = 3;
        
        return Self {
            paddle1: Rectangle {
                id: 1, 
                upper_screen_bound: y_size as i16,
                speed: paddle_speed,
                size: Vector2i {
                    x: 1,
                    y: 5,
                },
                position: Vector2i { 
                    x: 0,
                    y: y_size as i16 / 2,
                },
            },
            paddle2: Rectangle {
                id: 2,
                speed: paddle_speed,
                upper_screen_bound: y_size as i16,
                size: Vector2i {
                    x: 1,
                    y: 5,
                },
                position: Vector2i { 
                    x: x_size as i16 - 2,
                    y: y_size as i16 / 2,
                },
            },
            stdin: None,
            stdout: None,

            ended: false,
            ball: Ball::new(x_size/2, y_size/2),
            grid: Grid::new(x_size, y_size),
            frame_interval: std::time::Duration::from_millis(25),
            score_info_interval: std::time::Duration::from_millis(2000),
            score: vec![0,0],
            tx,
            rx,
        } 
    }


    fn clear(&self) { 
        print!("{}", termion::clear::All);
    }

    pub fn is_ended(&self) -> bool { 
        return self.ended;
    }

    pub fn draw_helping_trail(&mut self) {
        let pos = self.ball.position;
        let mov = self.ball.movement;


        self.ball.draw_as_small = true;
        while self.ball.scored == '-' {
            self.ball.move_shape();
            self.ball.draw(&mut self.grid);
        }
        self.ball.draw_as_small = false;

        self.ball.position = pos;
        self.ball.movement = mov;
        self.ball.scored = '-';
    }

    // pub fn process_keys(&mut self) { 
    //     let input = self.stdin.next();

    //     while !self.stdin.next().is_none() {
    //         self.stdin.next();
    //     }

    //     if let Some(Ok(key)) = input {
    //         match key {
    //             termion::event::Key::Char('q') => {
    //                 self.end()
    //             },
    //             termion::event::Key::Up => {
    //                 self.paddle1.move_paddle(-1);
    //             },
    //             termion::event::Key::Down => {
    //                 self.paddle1.move_paddle(1);
    //             },
    //             termion::event::Key::Char('w') => {
    //                 self.paddle2.move_paddle(-1);
    //             },
    //             termion::event::Key::Char('s') => {
    //                 self.paddle2.move_paddle(1);
    //             },
    //             _ => {}
    //         }
    //     }
    // }

    pub fn check_scores(&mut self) { 
        if self.ball.scored != '-' {
            println!("{}", termion::clear::All);

            if self.ball.scored == '1' {
                self.score[0] += 1;
            }   
            else{
                self.score[1] += 1;
            }

            self.ball.position = Vector2i {
                x: self.grid.size.x/2,
                y: self.grid.size.y/2,
            };

            println!("{} vs {}", self.score[0], self.score[1]);

            self.ball.scored = '-';

            thread::sleep(self.score_info_interval);
        }
    }

    fn load(&mut self, recv: GameInfo) {
        self.paddle1.position.y = recv.paddle1_pos;
        self.paddle2.position.y = recv.paddle2_pos;
        

        //todo check if data match
        //`self.ball.position.x = r.ball_pos.x;
        //`self.ball.position.y = r.ball_pos.y;
        //`self.ball.movement.x = r.ball_speed.x;
        //self.ball.movement.y = r.ball_speed.y;
    }

    fn dump_to_info(&self) -> GameInfo {
        GameInfo {
            ball_pos : self.ball.position,
            ball_speed: self.ball.movement,
            paddle1_pos: self.paddle1.position.y,
            paddle2_pos: self.paddle2.position.y,
        }
    }

    pub fn loop_logic(&mut self) {

        //self.clear();
        //self.process_keys();

        let result = self.rx.recv();

        match result {
            Ok(game_info) => {
                self.load(game_info);
            },
            Err(err) => {
                println!("{} err!", err);
                std::process::exit(1);
            }
        }

        self.paddle1.draw(&mut self.grid);
        self.paddle2.draw(&mut self.grid);
        self.ball.draw(&mut self.grid);
        self.ball.move_shape();

        self.check_scores();

        //self.grid.draw();
        self.grid.clear();

        let _result = self.tx.send(self.dump_to_info());

    }
    
    fn end(&mut self) { 
        self.ended = true;
    }
}
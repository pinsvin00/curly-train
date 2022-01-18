

use crate::Rectangle;
use crate::Grid;
use crate::Ball;
use crate::Vector2i;
use crate::GameState;
use crate::gameinfo::GameInfo;
use crate::GCSignalType;
use std::sync::mpsc::{Receiver, Sender};
use crate::Drawable;
use termion::raw::IntoRawMode;
use std::io::{stdout};
use termion::input::TermRead;

use std::thread;

pub struct Game{
    paddles: [Rectangle; 2],
    ball: Ball,
    grid: Grid,
    score_info_interval: std::time::Duration,

    ended: bool,
    score: Vec<u16>,

    stdin : termion::input::Keys<termion::AsyncReader>,
    _stdout: termion::raw::RawTerminal<std::io::Stdout>,

    paddle_num : u8,

    rx : Receiver<GameState>,
    tx : Sender<GameState>,
    
    signal: GCSignalType,
    game_info: GameInfo,
}

impl Game {
    pub fn new(game_info: GameInfo, paddle_num : u8, tx : Sender<GameState>, rx: Receiver<GameState>) -> Self { 
        
        let x_size = game_info.board_size.x as i16;
        let y_size = game_info.board_size.y as i16;

        let paddle_speed = game_info.paddle_speed as i16;

        let ball_mov = game_info.ball_speed.clone();

        return Self {
            game_info,
            paddles: [
                Rectangle {
                    id: 1, 
                    upper_screen_bound: y_size,
                    speed: paddle_speed,
                    size: Vector2i {
                        x: 1,
                        y: 5,
                    },
                    position: Vector2i { 
                        x: 0,
                        y: y_size  / 2,
                    },
                },
                Rectangle {
                    id: 2,
                    speed: paddle_speed,
                    upper_screen_bound: y_size,
                    size: Vector2i {
                        x: 1,
                        y: 5,
                    },
                    position: Vector2i { 
                        x: x_size - 2,
                        y: y_size / 2,
                    },
                },
            ],


            stdin: termion::async_stdin().keys(),
            _stdout: stdout().into_raw_mode().unwrap(),
            paddle_num,
            ended: false,
            ball: Ball::new(x_size as usize /2, y_size as usize/2, ball_mov),
            grid: Grid::new(x_size as usize,    y_size as usize),
            score_info_interval: std::time::Duration::from_millis(2000),
            score: vec![0,0],
            signal: GCSignalType::Default,
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
            self.move_ball();
            self.ball.draw(&mut self.grid);
        }
        self.ball.draw_as_small = false;

        self.ball.position = pos;
        self.ball.movement = mov;
        self.ball.scored = '-';
    }

    pub fn process_keys(&mut self) { 
        let input = self.stdin.next();

        while !self.stdin.next().is_none() {
            self.stdin.next();
        }

        let paddle_num = (self.paddle_num - 1) as usize;
        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Char('q') => {
                    self.end();
                },
                termion::event::Key::Char('w') => {
                    self.paddles[paddle_num].move_paddle(-1);  
                },
                termion::event::Key::Char('s') => {
                    self.paddles[paddle_num].move_paddle(1);  
                },
                _ => {}
            }
        }
    }

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

    fn load(&mut self, recv: GameState) -> GCSignalType {

        let rbv = recv.ball_pos;
        if rbv.x != self.ball.position.x && rbv.y != self.ball.position.y {
            print!("Possible desync on ball position? Aborted loading data.\r\n");
            return GCSignalType::Desync;
        }


        let mut diffs = [0,0];
        diffs[0] = (self.paddles[0].position.y - recv.paddle1_pos).abs();
        diffs[1] = (self.paddles[1].position.y - recv.paddle2_pos).abs();


        for diff in diffs {
            if diff > self.game_info.paddle_speed as i16 {
                print!("Possible desync on paddle position? Aborted loading data.\r\n");
                //return GCSignalType::Desync;
            }
        }

        if self.paddle_num == 2 {
            self.paddles[0].position.y = recv.paddle1_pos;
        }
        else {
            self.paddles[1].position.y = recv.paddle2_pos;
        }



        return GCSignalType::Default;

    }

    fn gameinfo_adapter(&self) -> GameState {
        GameState {
            ball_pos : self.ball.position,
            ball_speed: self.ball.movement,
            paddle1_pos: self.paddles[0].position.y,
            paddle2_pos: self.paddles[1].position.y,
            terminate: false,
            sig_type: self.signal,
        }
    }

    pub fn check_linear_overlap(&self) -> bool {
        for y in 0..self.ball.movement.y {
            for x in 0..self.ball.movement.x {
                let pos = Vector2i {
                    x: self.ball.position.x + x,
                    y: self.ball.position.y + y,
                };
                if self.grid.get(pos.x as usize, pos.y as usize) != ' ' {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn move_ball(&mut self) {

        self.ball.position.x += self.ball.movement.x;
        self.ball.position.y += self.ball.movement.y;
            
        let x_bounds = [0, self.game_info.board_size.x];
        let y_bounds = [0, self.game_info.board_size.y];

        let right_score = self.ball.position.x < x_bounds[0] as i16;
        let left_score = self.ball.position.x > x_bounds[1] as i16;

        //check if paddle is overlapped

        if left_score || right_score  {
            if !self.check_linear_overlap() {
                if left_score {
                    self.ball.scored = '1';
                }
                else {
                    self.ball.scored = '2';
                }
            }
            self.ball.movement.x *= -1;
        }

        for y_bound in y_bounds {
            if (self.ball.position.y - y_bound as i16).abs() == 0 { 
                self.ball.movement.y *= -1;
                break;
            } 
        } 
    }

    pub fn loop_logic(&mut self) {

        self.clear();
        
        let _result = self.tx.send(self.gameinfo_adapter());
        let result = self.rx.recv();

        match result {
            Ok(game_info) => {
                if game_info.terminate {
                    self.end();
                    return;
                }
                self.signal = self.load(game_info);
            },
            Err(_) => {
                self.end();
                return;
            }
        }

        self.process_keys();

        for paddle in &mut self.paddles {
            paddle.draw(&mut self.grid);
        }

        if self.game_info.draw_helping_trail {
            self.draw_helping_trail();
        }

        self.ball.draw(&mut self.grid);
        self.move_ball();
        self.check_scores();


        println!("p1 {:?}, p2: {:?}, ballm {:?}, ballp {:?} , Player : {}, Desync\r",
         self.paddles[0].position, self.paddles[1].position, self.ball.movement, self.ball.position, self.paddle_num);

        self.grid.draw();
        self.grid.clear();

    }
    
    fn end(&mut self) {
        self.clear();
        self.ended = true;
    }
}
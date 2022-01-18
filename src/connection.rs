use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::{BufReader};
use std::io::prelude::*;
use std::sync::mpsc::{Sender, Receiver};

use crate::gameinfo::GameInfo;
use crate::gamestate::GameState;
use crate::switch_to_normal;
use std::fs;


pub struct Connection {
    stream: Option<TcpStream>,
    ip_addr: Option<String>,
    listening: bool,
    pub curr_game_info: GameState,
    tx : Sender<GameState>,
    rx : Receiver <GameState>
}

impl Connection {
    pub fn connector(ip_addr: String, tx: Sender<GameState>, rx : Receiver <GameState>) -> Option<Connection> {
        return Some(Self {
            ip_addr : Some(ip_addr),
            curr_game_info : GameState::new(),
            listening: false,
            stream : None,
            tx,
            rx,
        })
    }

    pub fn listener(tx: Sender<GameState>, rx : Receiver <GameState>) -> Connection {
        return Self {
            ip_addr: None,
            curr_game_info : GameState::new(),
            listening: true,
            stream: None,
            tx,
            rx,
        }
    }

    pub fn acquire(&mut self, conn_dispatcher : Sender<GameInfo>) {
        if !self.listening { 

            let addr = self.ip_addr.as_ref();
            let result = TcpStream::connect(String::from("0.0.0.0:7999"));
            match result {
                Ok(stream) => {
                    self.stream = Some(stream);
                },
                Err(err) => {
                    print!("\r\nCannot connect to the socket, be sure that you've entered the valid ip address\r\n {}\r\n", err);
                    switch_to_normal();
                    std::process::exit(1);
                },

            }

            let mut reader = BufReader::new(self.stream.as_ref().unwrap());

            let mut host_info_raw = String::new();
            reader.read_line(&mut host_info_raw).unwrap();
            host_info_raw.pop();

            let game_info = GameInfo::parse(host_info_raw);

            conn_dispatcher.send(game_info).unwrap();
            self.loop_game_tick();
        }

        else { 
            let result = TcpListener::bind("127.0.0.1:7999");
            let listener;
            match result {
                Ok(lis) => {
                    listener = lis;
                },
                Err(err) => {
                    print!("\r\nError occured whilst listening!\r\n {} \r\n", err);
                    switch_to_normal();
                    std::process::exit(1);
                }
            }

            for s in listener.incoming() {

                self.stream = Some(s.unwrap());

                let mut stream = self.stream.as_ref().unwrap();
                
                let conny_path = String::from(std::env::current_dir().unwrap().to_str().unwrap()) + "/conny.sp";
                let mut conn_data_raw = fs::read_to_string(conny_path).expect("Cannot open conny.sp!");
                conn_data_raw.push_str("\n");


                stream.write(conn_data_raw.as_bytes()).unwrap();


                conn_data_raw.pop();
                let game_info = GameInfo::parse(conn_data_raw);
                conn_dispatcher.send(game_info).unwrap();

 
                self.loop_game_tick();
            }

        }
    }

    
    pub fn loop_game_tick(&mut self) {
        loop {
            let result = self.rx.recv();
            match result {
                Ok(gd) => {
                    self.curr_game_info = gd;
                },
                Err(err) => {
                    println!("err! {}", err);
                    std::process::exit(1)
                }
            }

            let packet = self.curr_game_info.str();
            let mut stream = self.stream.as_ref().unwrap();
            let mut reader = BufReader::new(stream);



            match stream.write(packet.as_bytes()) {
                Ok(_) => {},
                Err(_) => {
                    self.curr_game_info.terminate = true;
                    self.tx.send(self.curr_game_info).unwrap();
                    break;
                },
            }

            stream.flush().unwrap();
            let mut data_recv = String::new();
            reader.read_line(&mut data_recv).unwrap();
            data_recv.pop();

            self.curr_game_info.update(data_recv);
            self.tx.send(self.curr_game_info).unwrap();

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    } 
}

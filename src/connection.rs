use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::sync::mpsc::{Sender, Receiver};

use crate::connectioninfo::ConnectionInfo;
use crate::gameinfo::GameInfo;
use crate::Game;
use crate::switch_to_normal;
use crate::Vector2i;
use crate::utils::ParseInfo;

const PROTO_VER : &str = "1.0.0";

pub struct Connection {
    stream: Option<TcpStream>,
    ip_addr: Option<String>,
    listening: bool,
    conn_info: Option<ConnectionInfo>,
    pub curr_game_info: GameInfo,
    thread_handle: Option<std::thread::JoinHandle<()>>,
    tx : Sender<GameInfo>,
    rx : Receiver <GameInfo>
}
impl Connection {
    pub fn connector(ip_addr: String, tx: Sender<GameInfo>, rx : Receiver <GameInfo>) -> Option<Connection> {
        return Some(Self {
            thread_handle: None,
            ip_addr : Some(ip_addr),
            conn_info : None,
            curr_game_info : GameInfo::new(),
            listening: false,
            stream : None,
            tx,
            rx,
        })
    }

    pub fn listener(tx: Sender<GameInfo>, rx : Receiver <GameInfo>) -> Connection {
        return Self {
            thread_handle: None,
            ip_addr: None,
            curr_game_info : GameInfo::new(),
            conn_info: None,
            listening: true,
            stream: None,
            tx,
            rx,
        }
    }

    pub fn acquire(&mut self) {
        if !self.listening { 
            let addr = self.ip_addr.as_ref();
            self.stream = Some(TcpStream::connect(addr.unwrap()).unwrap());
            self.handshake();
            loop {
                self.loop_game_tick();
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }

        else { 
            let result = TcpListener::bind("127.0.0.1:7999");
            let mut listener : TcpListener;

            match result {
                Ok(lis) => {
                    listener = lis;
                },
                Err(err) => {
                    print!("err! : {} \r\n", err);
                    switch_to_normal();
                    std::process::exit(1);
                }
            }

            for stream in listener.incoming() {
                let s = stream.unwrap();
                self.stream = Some(s);
                self.handshake();
                self.loop_game_tick();
            }

        }
    }

    pub fn handshake(&mut self) -> ConnectionInfo { 
        let conn_data = format!("{};{};{};{};{}\n", PROTO_VER, "test" ,"3" ,"2" ,"1" );
        let mut stream = self.stream.as_ref().unwrap();
        let mut reader = BufReader::new(stream);

        stream.write(conn_data.as_bytes()).unwrap();

        let conn_parse_info = ConnectionInfo::new(conn_data);
        let mut conn_info;
        match conn_parse_info { 
            ParseInfo::Ok(_conn) => {
                conn_info = _conn;
            },
            ParseInfo::Err => {
                std::process::exit(1);
            },
        }

        let mut response = String::new();
        reader.read_line(&mut response).expect("Provided invalid data...");
        response.pop();

        return conn_info;
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
                    self.curr_game_info.end_game = true;
                    self.tx.send(self.curr_game_info);
                    break;
                },
            }
            stream.flush().unwrap();

            let mut data_recv = String::new();

            reader.read_line(&mut data_recv).unwrap();
            data_recv.pop();
            self.curr_game_info.update(data_recv);


            let result = self.tx.send(self.curr_game_info);
            match result {
                Err(e) => {
                    println!("Critical tx error, terminating program. {}", e);
                    std::process::exit(2);
                },
                Ok(_) => {},
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    } 
}

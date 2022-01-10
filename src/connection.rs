use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use crate::connectioninfo::ConnectionInfo;
use crate::gameinfo::GameInfo;
use crate::utils::ParseInfo;

const PROTO_VER : &str = "1.0.0";
pub struct Connection<'a> {
    reader: Option<BufReader<TcpStream>>,
    stream: Option<TcpStream>,
    ip_addr: Option<String>,
    listening: bool,
    conn_info: Option<ConnectionInfo<'a>>,
    curr_game_info: Option<GameInfo>
}

impl<'a> Connection<'a> {
    fn new(ip_addr: String) -> Option<Connection<'a>> {
        return Some(Self {
            ip_addr : Some(ip_addr),
            conn_info : None,
            curr_game_info : None,
            listening: false,
            reader : None,
            stream : None,
        })
    }

    pub fn acquire_connectin(&mut self) {
        if !self.listening { 
            let addr = self.ip_addr.as_ref();
            self.stream = Some(TcpStream::connect(addr.unwrap()).unwrap());
        }
        else { 
            let listener = TcpListener::bind("127.0.0.1:7999").unwrap();
            for stream in listener.incoming() {
                let s = stream.unwrap();
                self.stream = Some(s);
                break;
            }

        }
    }

    pub fn handshake(&mut self) { 
        let conn_data = format!("{};{};{};{};{}\n", PROTO_VER,"test","3","2","1");
        let mut stream = self.stream.as_ref().unwrap();
        let mut reader = BufReader::new(stream);

        stream.write(conn_data.as_bytes()).unwrap();

        let mut response = String::new();
        reader.read_line(&mut response);
        response.pop();

        if response == "OK" {
            println!("Connection sucesfully establisehd...");
        } else {
            println!("Connection failed! Server rseponded with {}", response);
            return; 
        }

        let conn = ConnectionInfo::new(&conn_data);
        match conn {
            ParseInfo::Ok(conn_i) => {
                self.conn_info = Some(conn_i);             
            },
            ParseInfo::Err => {
                println!("Couldnt establish connection");
            }
        }
    }
    
    pub fn game_tick(&mut self) {
        let packet = String::from("sussy baka");
        let mut stream = self.stream.as_ref().unwrap();
        let mut reader = BufReader::new(stream);

        stream.write(packet.as_bytes()).unwrap();
        stream.flush().unwrap();

        let mut data_recv = String::new();
        reader.read_line(&mut data_recv).unwrap();
        
        print!("recv {}", data_recv);

        std::thread::sleep(std::time::Duration::from_secs(1));
    } 
}



// pub fn listen_conn() {
//     let listener = TcpListener::bind("127.0.0.1:7999").unwrap();
//     let mut connected = false;
//     for stream in listener.incoming() {
//         let mut s = stream.unwrap();
//         if !connected {
//             connected = true;
//             handle_client(&mut s);
//         }
//     }
// }

// pub fn connect() {
//     let mut stream = TcpStream::connect("127.0.0.1:7999").unwrap();
//     let mut stream_copy = stream.try_clone().unwrap(); 
//     let mut reader = BufReader::new(&mut stream_copy);

    
// }


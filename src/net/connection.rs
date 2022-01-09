use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::{BufReader, BufWriter, Write};
use std::io::prelude::*;
use std::thread;
use std::time::Duration;


struct Connection {
    reader: BufReader,
    stream: TcpStream,
    listening: bool,
    conn_info: Option<ConnectionInfo>,
    curr_game_info: Option<GameInfo>
}

impl Connection {
    fn new(ip_adrr: String) -> Option<Connection> {
        let stream = TcpStream::connect(ip_addr);
        let mut stream_copy = stream.try_clone().unwrap();
        let mut reader = BufReader::new(&mut stream_copy);
        
        let conn_data = format!("{};{};{};{};{}\n", PROTO_VER,"test","3","2","1");
        stream.write(conn_data.as_bytes()).unwrap();

        let mut response = String::new();
        reader.read_line(&mut response);
        repsonse.pop();

        if response == "OK" {
            println!("Connection sucesfully establisehd...");
        } else {
            println!("Connection failed! Server rseponded with {}", response);
            return None;
        }

        let conn_info = ConnectionInfo::new(conn_data);
        
        return Self {
            reader,
            stream,
            listening: false,
            conn_info,
            curr_game_info: None,
        }
    },
    
    pub fn game_tick(&mut self) {
        let packet = String::from("sussy baka");

        self.stream.write(packet.as_bytes()).unwrap();
        self.stream.flush().unwrap();

        reader.read_line(&mut packet).unwrap();
        
        print!("{} recv", packet);

        std::thread::sleep(std::time::Duration::from_secs(1));
    } 
}


fn handle_client(mut stream :&TcpStream){
    let stream_copy = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    let mut handshake = String::new();
    
    reader.read_line(&mut handshake).unwrap();
    stream.flush().unwrap();
    
    handshake.pop();
    let game_info = ConnectionInfo::new(&handshake);

    let ok = String::from("OK\n");
    let nok = String::from("NOK\n");

    let mut _conn: ConnectionInfo;
    match game_info {
        ParseInfo::Err => {
            stream.write(nok.as_bytes()).unwrap();
            return;
        },
        ParseInfo::Ok(data) => {
            stream.write(ok.as_bytes()).unwrap();
            _conn = data;
        },
    }

    println!("connection succesful...");
    let mut _game_processed = true;


    loop {
        if _game_processed {

            stream.write(packet.as_bytes()).unwrap();
            stream.flush().unwrap();

            reader.read_line(&mut packet).unwrap();
            print!("{} recv", packet);

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }


}



pub fn listen_conn() {
    let listener = TcpListener::bind("127.0.0.1:7999").unwrap();
    let mut connected = false;
    for stream in listener.incoming() {
        let mut s = stream.unwrap();
        if !connected {
            connected = true;
            handle_client(&mut s);
        }
    }
}

pub fn connect() {
    let mut stream = TcpStream::connect("127.0.0.1:7999").unwrap();
    let mut stream_copy = stream.try_clone().unwrap(); 
    let mut reader = BufReader::new(&mut stream_copy);

    
}



use crate::vector::Vector2i;

pub struct GameInfo {
    pub _proto_ver: String,
    pub _ip_addr: String,
    pub is_host: bool,
    pub paddle_speed: u8,
    pub ball_speed: Vector2i,
    pub board_size: Vector2i,
    pub draw_helping_trail: bool,
}

impl GameInfo {
    pub fn parse(data_raw: String) -> GameInfo {

        let data : Vec<String> = data_raw.split(';').map(String::from).collect();

        if data.len() != 7 {
            print!("Invalid conny.sp content.");
            std::process::exit(2);
        }


        let mut info = GameInfo {
            is_host: false,
            ball_speed: Vector2i { 
                x: 1,
                y: 2,
            },
            board_size: Vector2i {
                x: 270,
                y: 45,
            },
            _ip_addr: String::from(""),
            paddle_speed: 0,
            _proto_ver: String::from("1"),
            draw_helping_trail: false,
        };

    
        info._proto_ver   = data[0].clone();

        info.ball_speed.x = data[1].parse::<i16>().unwrap();
        info.ball_speed.y = data[2].parse::<i16>().unwrap();

        info.board_size.x = data[3].parse::<i16>().unwrap();
        info.board_size.y = data[4].parse::<i16>().unwrap();

        info.paddle_speed = data[5].parse::<u8>().unwrap();

        info.draw_helping_trail = data[6].parse::<u8>().unwrap() == 1;


        return info;
    }
}

impl ToString for GameInfo {
    fn to_string(&self) -> String {
        return String::from(
            format!("{};{};{};{}\n", self._proto_ver, self.ball_speed.to_string(), self.board_size.to_string(), self.paddle_speed)
        );
    }
}

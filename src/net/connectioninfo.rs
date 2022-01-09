pub const PROTO_VER: &str = "1.00";
pub struct ConnectionInfo<'a> {
    proto_ver: &'a str,
    ip_addr: &'a str,
    paddle_speed: u8,
    ball_speed: Vector2i,
}

pub impl<'a> ConnectionInfo<'a> {
    fn new(data_raw: &'a String) -> ParseInfo<ConnectionInfo> {
        let data : Vec<&str> = data_raw.split(';').collect();

        if data.len() < 4 {
            return ParseInfo::Err;
        }

        let x = data[HandShakeInfo::BallSpeedX as usize].parse::<i8>().unwrap();
        let y = data[HandShakeInfo::BallSpeedY as usize];
        println!("y : {}", y.len());


        let info = Self {
            ball_speed: Vector2i { 
                x: 1,//data[HandShakeInfo::BallSpeedX as usize].parse::<i8>().unwrap(),
                y: 2,// data[HandShakeInfo::BallSpeedY as usize].parse::<i8>().unwrap(),
            },
            ip_addr: data[HandShakeInfo::ConnectorIP as usize],
            paddle_speed: data[HandShakeInfo::PaddleSpeed as usize].parse::<u8>().unwrap(),
            proto_ver: data[HandShakeInfo::ProtocolVer as usize],
        };

        if info.proto_ver != PROTO_VER { 
            return ParseInfo::Err;
        }

        return ParseInfo::Ok(info);
    }
}

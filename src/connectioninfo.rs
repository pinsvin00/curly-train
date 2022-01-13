
use crate::vector::Vector2i;
use crate::utils::{ParseInfo, HandShakeInfo};

pub const PROTO_VER: &str = "1.00";
pub struct ConnectionInfo {
    proto_ver: String,
    ip_addr: String,
    paddle_speed: u8,
    ball_speed: Vector2i,
}

impl ConnectionInfo {
    pub fn new(mut data_raw: String) -> ParseInfo<ConnectionInfo> {
        data_raw.pop();
        let data : Vec<String> = data_raw.split(';').map(String::from).collect();

        if data.len() < 4 {
            return ParseInfo::Err;
        }

        let x = data[HandShakeInfo::BallSpeedX as usize].parse::<i8>().unwrap();

        let y = data[HandShakeInfo::BallSpeedY as usize].parse::<i8>().unwrap();


        let info = Self {
            ball_speed: Vector2i { 
                x: 1,//data[HandShakeInfo::BallSpeedX as usize].parse::<i8>().unwrap(),
                y: 2,// data[HandShakeInfo::BallSpeedY as usize].parse::<i8>().unwrap(),
            },
            ip_addr: data[HandShakeInfo::ConnectorIP as usize].clone(),
            paddle_speed: data[HandShakeInfo::PaddleSpeed as usize].parse::<u8>().unwrap(),
            proto_ver: data[HandShakeInfo::ProtocolVer as usize].clone(),
        };


        return ParseInfo::Ok(info);
    }
}

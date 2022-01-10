

use crate::vector::Vector2i;

pub struct GameInfo { 
    paddle1_pos: i16,
    paddle2_pos: i16,
    ball_pos: Vector2i,
    ball_speed: Vector2i,
}

enum GameInfoIndex { 
    Paddle1PosY = 0,
    Paddle2PosY = 1,
    BallPosX    = 2,
    BallPosY    = 3,
    BallSpeedX  = 4,
    BallSpeedY  = 5,
}

fn get_info_split(info_split : &Vec<&str>, index: i16) -> i16 {
    return info_split[index as usize].parse::<i16>().unwrap();
}

impl GameInfo {
    fn new() -> GameInfo {
        return GameInfo {
            paddle1_pos: 0,
            paddle2_pos: 0,
            ball_pos: Vector2i {
                x: 0,
                y: 0,
            },
            ball_speed: Vector2i {
                x: 0,
                y: 0,
            }
        };

    }

    fn update(&mut self, info_raw: &String) {
        let info_split: Vec<&str> = info_raw.split(";").collect();

        self.paddle1_pos = get_info_split(&info_split, GameInfoIndex::Paddle1PosY as i16); 
        self.paddle2_pos = get_info_split(&info_split, GameInfoIndex::Paddle2PosY as i16); 
        
        self.ball_pos.x = get_info_split(&info_split, GameInfoIndex::BallPosX as i16); 
        self.ball_pos.y = get_info_split(&info_split, GameInfoIndex::BallPosY as i16);

        self.ball_speed.x = get_info_split(&info_split, GameInfoIndex::BallSpeedY as i16); 
        self.ball_speed.y = get_info_split(&info_split, GameInfoIndex::BallSpeedY as i16); 
    }
}

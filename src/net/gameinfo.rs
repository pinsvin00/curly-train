
pub struct GameInfo { 
    paddle1_pos: i8,
    paddle2_pos: i8,
    ball_pos: Vector2i,
    ball_speed: Vector2i,
}

enum GameInfoIndex { 
    Paddle1PosY,
    Paddle2PosY,
    BallPosX,
    BallPosY,
    BallSpeedX,
    BallSpeedY,
}

fn get_info_split(info_split : Vec<&str>, index: i8) -> i8 {
    return info_split[index as usize].parse::<i8>().unwrap();
}

impl GameInfo {
    fn new() {
        let mut game_info = GameInfo {
            paddle1_pos: 0
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

    },
    fn update(&mut self, info_raw: &String) {
        let info_split: Vec<&str> = info_raw.split(";").collect();

        self.paddle1_pos.y = get_info_split(info_split, GameInfoIndex::Paddle1PosY); 
        self.paddle2_pos.y = get_info_split(info_split, GameInfoIndex::Paddle2PosY); 
        
        self.ball_pos.x = get_info_split(info_split, GameInfoIndex::BallPosX); 
        self.ball_pos.y = get_info_split(info_split, GameInfoIndex::BallPosY);

        self.ball_speed.x = get_info_split(info_split, GameInfoIndex::BallSpeedY); 
        self.ball_speed.y = get_info_split(info_split, GameInfoIndex::BallSpeedY); 
    }


}

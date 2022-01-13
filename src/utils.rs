use crate::X_SIZE;
use crate::Y_SIZE;
use std::io::stdout;
use crate::Vector2i;

pub fn is_position_valid(grid : &Vec<Vec<char>>, position: Vector2i) -> bool {

    let x = position.x as usize;
    let y = position.y as usize;

    if grid.len() == 0 {
        return false;
    } 

    if x < grid[0].len() && y < grid.len() {
        return true;
    }
    return false;
}


pub fn get(array: & Vec<Vec<char>>, x: usize, y: usize) -> char {
    if x < X_SIZE && y < Y_SIZE {
        return array[x][y];
    }
    return '!';
}

pub fn switch_to_normal() {
    print!("Switching back terminal mode to normal...\r\n");
    print!(
        "{}{}\r\n",
        termion::cursor::Show,
        termion::clear::AfterCursor
    );
}

pub enum ParseInfo<T> {
    Ok(T),
    Err,
}


pub enum HandShakeInfo {
    ProtocolVer = 0,
    ConnectorIP = 1,
    PaddleSpeed = 2,
    BallSpeedX  = 3,
    BallSpeedY  = 4,
}

use crate::drawable::Drawable;
use crate::Vector2i;

use crate::Grid;


pub struct Rectangle {
    pub size: Vector2i,
    pub position: Vector2i,
    pub id: i16,
    pub speed: i16,
    pub upper_screen_bound: i16, 
}

impl Rectangle {
    pub fn move_paddle(&mut self, dir: i16) {

        let logic_case = if dir == - 1 {
            (0, dir)
        } else {
            (self.upper_screen_bound, 0)
        };

        let bound = logic_case.0;
        let size_factor = logic_case.1;
        let dis_bound = ((self.position.y + (self.size.y * size_factor)) - bound).abs();

        if dis_bound == 0 {
            return;
        }
        else if dis_bound < self.speed as i16 {
            self.position.y += dis_bound * dir;
        }
        else{ 
            self.position.y += self.speed * dir;
        } 
        
    }
}

impl Drawable for Rectangle {

    fn draw(&mut self, grid: &mut Grid) {
        let mut x = self.position.x;
        let mut y = self.position.y;
        let id_option = char::from_digit(self.id as u32, 10);
        let char_id = id_option.unwrap();

        for _y_dec in 0..self.size.y {
            grid.set(x as usize, y as usize, char_id);
            y -= 1;
        }
        for _x_incr in 0..self.size.x {
            grid.set(x as usize, y as usize, char_id);
            x += 1;
        }
        for _y_dec in 0..self.size.y {
            grid.set(x as usize, y as usize, char_id);
            y += 1;
        }
        for _x_incr in 0..self.size.x {
            grid.set(x as usize, y as usize, char_id);
            x -= 1;
        }
    }   
}

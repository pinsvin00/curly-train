
use crate::Vector2i;

pub struct Grid {
    pub array: Vec<Vec<char>>,
    pub size: Vector2i,
}

impl Grid {
    pub fn get(&self, x: usize, y: usize) -> char {
        if x < self.size.x as usize && y < self.size.y as usize {
            return self.array[x][y];
        }
        return '!';
    }

    pub fn set(&mut self, x: usize, y: usize, val: char) {
        let valid = self.get(x, y);
        if valid != '!' {
            self.array[x][y] = val;
        }
    }


    pub fn new(x: usize, y: usize) -> Self {
        Self {
            array: vec![vec![' '; y]; x],
            size: Vector2i{
                x: x as i16,
                y: y as i16,
            },
        }
    }

    pub fn clear(&mut self) { 
        let size = self.size;
        self.array = vec![vec![' '; size.y as usize] ; size.x as usize];
    }
    
    fn draw_border(&self) { 
        for _x in 0..self.size.x { 
            print!("#");
        }

        print!("\r\n");
    }

    pub fn draw(&mut self) {
        let x_size = self.size.x;
        let y_size = self.size.y;

        self.draw_border();
        for y in 0..y_size {
            for x in 0..x_size {
                let mut value = self.get(x as usize, y as usize);
                if value == '!' { 
                    value = ' ';
                }

                print!("{}", value);
            }
            print!("\r\n");
        }
        self.draw_border();
    }

}
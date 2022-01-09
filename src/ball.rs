use crate::Vector2i;


use crate::X_SIZE;
use crate::Y_SIZE;
use crate::drawable::Drawable;
use crate::get;
use crate::is_position_valid;
use crate::Grid;

fn get_shape(grid : &Vec<Vec<char>>, position: Vector2i) -> Vec<Vector2i> { 

    let mut visited = vec![vec![false; grid[0].len()]; grid.len() ];

    visited[position.x as usize][position.y as usize] = true;

    let mut positions = vec![position];


    let shape_value = get(grid, position.x as usize, position.y as usize);

    let mut shape = vec![position];

    while positions.len() != 0 {
        let dirs = [[1,0], [-1, 0], [0, 1], [0, -1]];
        for dir in dirs {
            let x = dir[0];
            let y = dir[1];
            let vector = Vector2i {
                x,y
            };

            if is_position_valid(grid,vector) {
                let curr_value = get(grid, vector.x as usize, vector.y as usize);

                let x = vector.x as usize;
                let y = vector.y as usize;

                if curr_value == shape_value && !visited[vector.x as usize][vector.y as usize] {
                    positions.push(vector);
                    shape.push(vector);
                    visited[x][y] = true;
                }
            }

        }
        positions.remove(0);
    }
    return shape;
}

    pub struct Ball {
        pub movement: Vector2i,
        pub position: Vector2i,
        collided: bool,
        collision_place: Vector2i,
        pub scored: char,
        pub draw_as_small: bool,
    }   

    impl Ball {
        pub fn new(x: usize, y: usize) -> Ball {
            Self { 
                position: Vector2i{
                    x:x as i16,
                    y:y as i16
                },
                draw_as_small: false,
                collided: false,
                scored: '-',
                movement: Vector2i{
                    x: -2,
                    y: 1
                },
                collision_place: Vector2i {
                    x: 0,
                    y:0,
                }
            }
        }
        pub fn move_shape(&mut self) {

            self.position.x += self.movement.x;
            self.position.y += self.movement.y;
            
            let x_bounds = [0, X_SIZE];
            let y_bounds = [0, Y_SIZE];

            let right_score = self.position.x < x_bounds[0] as i16;
            let left_score = self.position.x > x_bounds[1] as i16;

            if left_score || right_score  {
                if left_score {
                    self.scored = '1';
                }
                else {
                    self.scored = '2';
                }
                self.movement.x *= -1;
            }

            for y_bound in y_bounds {
                if (self.position.y - y_bound as i16).abs() == 0 { 
                    self.movement.y *= -1;
                    break;
                } 
            } 
        }
    }

    impl Drawable for Ball {
        fn draw(&mut self, grid: &mut Grid) {
            let pos = self.position;

            for dir in [[1,0], [-1, 0], [0, 1], [0, -1]] {
                let x = pos.x + dir[0];
                let y = pos.y + dir[1];

                let val = grid.get(x as usize, y as usize);

                if val != '#' && val != ' ' && val != '!' {
                    self.collided = true;
                }
                else if !self.draw_as_small{
                    grid.set( x as usize, y as usize, '#');
                }
            }

            if self.draw_as_small {
                grid.set(self.position.x as usize, self.position.y as usize, '#');
            }

            if self.collided == true {
                let collision_place = self.collision_place;
                let shape = get_shape(&grid.array, self.collision_place);
                let mut sum_y = 0;
                let mut count = 0;
                for v2i in shape {
                    count += 1;
                    sum_y += v2i.y;
                }
                
                let avg_y = sum_y / count;
                if collision_place.y < avg_y  { 
                    self.movement.y *= -1;

                }
                self.movement.x *= -1;
                self.collided = false;

            }




        }
    }


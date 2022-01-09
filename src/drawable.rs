
use crate::Grid;

pub trait Drawable {
    fn draw(&mut self, _grid: &mut Grid) {
    }
}

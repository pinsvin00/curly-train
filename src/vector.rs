
#[derive(Debug)]
pub struct Vector2i {
    pub x: i16,
    pub y: i16
}

pub struct Vector2f { 
    pub x: f32,
    pub y: f32,
}


impl ToString for Vector2i {
    fn to_string(&self) -> String {
        return String::from(format!("{};{}", self.x, self.y))
    }
}

impl Copy for Vector2f {}
impl Copy for Vector2i { }

impl Clone for Vector2f {
    fn clone(&self) -> Vector2f {
        *self
    }
}

impl Clone for Vector2i  {
    fn clone(&self) -> Vector2i {
        *self
    }
}

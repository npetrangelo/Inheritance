use crate::Shape;

#[derive(Debug)]
pub struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    pub fn new() -> Self {
        Rectangle {width: 1, height: 1}
    }
}

impl Shape for Rectangle {
    fn perimeter(&self) -> f32 {
        (2*self.width + 2*self.height) as f32
    }
}
use crate::{Shape, Vertex};

#[derive(Debug)]
pub struct Circle {
    center: Vertex,
    radius: f32
}

impl Circle {
    pub fn new() -> Self {
        Circle {center: Vertex(0, 0), radius: 1.0}
    }
}

impl Shape for Circle {
    fn perimeter(&self) -> f32 {
        2.0 * std::f32::consts::PI * self.radius
    }
}
use crate::{Shape, utils, Vertex};

#[derive(Debug)]
pub struct Pentagon {
    vertices: [Vertex; 5]
}

impl Shape for Pentagon {
    fn perimeter(&self) -> f32 {
        utils::perimeter(self.vertices.as_slice())
    }
}
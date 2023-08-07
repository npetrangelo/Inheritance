use crate::{Shape, utils, Vertex};

#[derive(Debug)]
pub struct Triangle {
    vertices: [Vertex; 3]
}

impl Triangle {
    pub fn new() -> Self {
        Triangle {vertices: [Vertex(0,0), Vertex(1,0), Vertex(0,1)]}
    }
}

impl Shape for Triangle {
    fn perimeter(&self) -> f32 {
        utils::perimeter(self.vertices.as_slice())
    }
}
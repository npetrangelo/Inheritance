mod triangle;
mod rectangle;
mod pentagon;
mod circle;

use std::ops::Sub;

mod utils {
    use crate::Vertex;

    fn dist(v1: &Vertex, v2: &Vertex) -> f32 {
        let Vertex(x, y) = v1 - v2;
        ((x + y) as f32).sqrt()
    }

    pub fn perimeter(vertices: &[Vertex]) -> f32 {
        let mut perimeter = dist(vertices.first().unwrap(), vertices.last().unwrap());
        for i in 1..vertices.len() {
            perimeter += dist(&vertices[i], &vertices[i-1])
        }
        perimeter
    }
}

#[derive(Debug)]
pub struct Vertex(i32, i32);

impl Sub for &Vertex {
    type Output = Vertex;

    fn sub(self, rhs: Self) -> Self::Output {
        Vertex(self.0 - rhs.0, self.1 - rhs.1)
    }
}

trait Shape {
    fn perimeter(&self) -> f32;
}

enum Shapes {
    Triangle(triangle::Triangle),
    Rectangle(rectangle::Rectangle),
    Pentagon(pentagon::Pentagon),
    Unknown(Box<dyn Shape>)
}

fn main() {
    println!("Hello, world!");
    let shapes = vec!(Shapes::Triangle(triangle::Triangle::new()), Shapes::Rectangle(rectangle::Rectangle::new()), Shapes::Unknown(Box::new(circle::Circle::new())));
}

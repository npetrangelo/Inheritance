mod triangle;
mod rectangle;
mod pentagon;
mod circle;

use std::ops::Sub;
use triangle::Triangle;
use rectangle::Rectangle;
use pentagon::Pentagon;
use circle::Circle;


mod utils {
    use crate::Vertex;

    fn dist(v1: &Vertex, v2: &Vertex) -> f32 {
        let Vertex(x, y) = v1 - v2;
        ((x*x + y*y) as f32).sqrt()
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
    Triangle(Triangle),
    Rectangle(Rectangle),
    Pentagon(Pentagon),
    Unknown(Box<dyn Shape>)
}

impl Shape for Shapes {
    fn perimeter(&self) -> f32 {
        match self {
            Shapes::Triangle(t) => {t.perimeter()}
            Shapes::Rectangle(r) => {r.perimeter()}
            Shapes::Pentagon(p) => {p.perimeter()}
            Shapes::Unknown(u) => {u.perimeter()}
        }
    }
}

fn main() {
    let shapes = vec!(Shapes::Triangle(Triangle::new()), Shapes::Rectangle(Rectangle::new()), Shapes::Unknown(Box::new(Circle::new())));
    println!("The perimeter of the shape is {:?}", shapes[0].perimeter());
}

# Inheritance
An exploration of solving the problems that inheritance solves, without inheritance.

OOP developers, like myself quite recently, may struggle to understand how to structure their code
without inheritance in Rust. This repository provides a simple, bare bones example of one approach,
identifying the three problems that inheritance traditionally solves, and solving them separately.

### Sharing a common interface
OOP developers are familiar with interfaces so I will not dwell on this too much, but Rust achieves
this functionality with its trait system.
```rust
trait Shape {
    fn perimeter(&self) -> f32;
}
```

### Code reuse
After replacing all class inheritance with trait inheritance, the OOP developer may ask about code reuse next.
In this repository, code reuse is achieved using functions, which have first class support in Rust. The common `perimeter()`
function is defined once in the `utils` module, and used in multiple of the `Shape` implementations.
```rust
mod utils {
    pub fn perimeter(vertices: &[Vertex]) -> f32 { ... }
}
```
```rust
impl Shape for Triangle {
    fn perimeter(&self) -> f32 {
        utils::perimeter(self.vertices.as_slice())
    }
}
```
```rust
impl Shape for Pentagon {
    fn perimeter(&self) -> f32 {
        utils::perimeter(self.vertices.as_slice())
    }
}
```

### Polymorphism
The last feature inheritance traditionally offers for us is polymorphism; providing
a child type wherever the parent type is expected. This *can* be achieved using only `dyn` type trait objects,
but one could rightly complain about the inefficiency under the hood, as these require the object to be behind
a reference of some kind. Furthermore, one may not want to accept *all* implementations of their trait, and only
support the ones they wrote. This is where Rust's enums enter the picture, as algebraic sum data types. Using an enum
that implements the same trait as the "children", all the "children's" implementations can be wired together and the
match expression does not have to be exposed.

In the event that you actually do want to handle implementations written by others,
this is still possible with a variant that takes a `dyn` type trait object. These implementations
will be less efficient as they cannot retroactively be included in the enum directly and must be behind
whichever reference type you use, but this is a necessary cost of using the enum you provide in this manner.
If they absolutely must have that performance anyway, they may write their own enums to include the types they want.
```rust
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
```

### Results
In the end, all of this allows us to store a collection of shapes, retrieve them, and call functions on them,
almost as if we still had inheritance. Note that the `Circle`, which is not included in the enum directly, may still
be included in the collection through the `Unknown` variant, as a `dyn` type trait object.
```rust
fn main() {
    let shapes = vec!(Shapes::Triangle(Triangle::new()), Shapes::Rectangle(Rectangle::new()), Shapes::Unknown(Box::new(Circle::new())));
    println!("The perimeter of the shape is {:?}", shapes[0].perimeter());
}
```

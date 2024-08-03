#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p lpxxn-creational_bin --bin lpxxn-creational-main```
///
/// ```cargo doc  --package lpxxn-creational_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package lpxxn-creational_bin ```
///
/// ## What
/// `Factory`
///
/// ## How
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///
/// //```rust,compile_fail,no_run,ignore
///
pub trait Shape {
fn area(&self) -> f64;
}
struct Circle {
    radius: f64,
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}
struct Rectangle {
    width: f64,
    height: f64,
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
pub struct ShapeFactory;
impl ShapeFactory {
    pub fn get_shape(shape_type: &str) -> Box<dyn Shape> {
        match shape_type {
            "circle" => Box::new(Circle { radius: 5.0 }),
            "rectangle" => Box::new(Rectangle { width: 10.0, height: 20.0 }),
            _ => panic!("Invalid shape"),
        }
    }
}
fn main() {
    let circle = ShapeFactory::get_shape("circle");
    println!("The area of the circle is {}", circle.area());

    let rectangle = ShapeFactory::get_shape("rectangle");
    println!("The area of the rectangle is {}", rectangle.area());
}

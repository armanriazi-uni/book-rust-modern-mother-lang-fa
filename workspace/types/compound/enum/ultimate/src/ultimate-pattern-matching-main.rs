/// Main
///
/// ## Commands
///
/// ```cargo run -q -p ultimate-pattern-matching_bin --bin ultimate-pattern-matching-main```
///
/// ```cargo doc --package ultimate-pattern-matching_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc --package ultimate-pattern-matching_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your enum/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
    }
    fn area(shape: Shape) -> f64 {
    match shape {
    Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
    Shape::Rectangle(width, height) => width * height,
    Shape::Triangle(a, b, c) => {
    let s = (a + b + c) / 2.0;
    (s * (s - a) * (s - b) * (s - c)).sqrt()
    }
    }
    }
    fn main() {
    println!("{:?}", area(Shape::Circle(32.0)));
    }
    // $ rustc pattern.rs && /pattern
    // Output: 3216.990877275948
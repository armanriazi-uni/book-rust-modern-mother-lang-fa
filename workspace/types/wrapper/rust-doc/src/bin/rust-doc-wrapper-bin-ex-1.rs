#![allow(dead_code, unused_variables)]

/// rust-doc-wrapper-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-wrapper_bin --bin rust-doc-wrapper-bin-ex-1```
///
/// ## What
/// `dewrapperure`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your wrapper/func name]
///
/// # Return
/// `assert:true`
///
/// ## Example
/// //```rust,compile_fail,ignore
///
#[derive(Debug)]
struct  Person {
    name: String,
    age: u8,
}

/// A unit wrapper
struct  Unit;

/// A tuple wrapper
struct  Pair(i32, f32);

/// A wrapper with two fields
struct  Point {
    x: f32,
    y: f32,
}

///// wrappers can be reused as fields of another wrapper
#[allow(dead_code)]
struct  Rectangle {
    /// A rectangle can be specified by where the top left and bottom right
    /// corners are in space.
    top_left: Point,
    bottom_right: Point,
}
///
///```no_run
/// Make a new point by using wrapper update syntax to use the fields of our other one
///let bottom_right = Point { x: 5.2, ..point };
///```
///
fn main() {
    /// Create wrapper with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    /// Print debug wrapper
    println!("{:?}", peter);


    //// Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    //// Access the fields of the point
    //println!("point coordinates: ({}, {})", point.x, point.y);

    /// Make a new point by using wrapper update syntax to use the fields of our other one
    let bottom_right = Point { x: 5.2, ..point };

    /// `bottom_right.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    /// Dewrapperure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // wrapper instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    /// Instantiate a unit wrapper
    let _unit = Unit;

    /// Instantiate a tuple wrapper
    let pair = Pair(1, 0.1);

    /// Access the fields of a tuple wrapper
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    /// Dewrapperure a tuple wrapper
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

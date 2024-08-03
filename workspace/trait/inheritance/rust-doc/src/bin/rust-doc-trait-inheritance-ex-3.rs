#![allow(dead_code, unused_variables)]

/// rust-doc-trait-inheritance-ex-3
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-doc-trait-inheritance_bin --bin rust-doc-trait-inheritance-ex-3```
///
/// ```cargo doc  --package rust-doc-trait-inheritance_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-trait-inheritance_bin ```
///
/// ## What
/// `Subtrait2`
///
/// ## How
/// `TODO`
///
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

fn main() {
    trait Shape { fn area(&self) -> f64; }
    trait Circle : Shape { fn radius(&self) -> f64; }

    struct UnitCircle;

    impl Shape for UnitCircle { fn area(&self) -> f64 { std::f64::consts::PI } }
    impl Circle for UnitCircle { fn radius(&self) -> f64 { 1.0 } }

    let circle = UnitCircle;
    let circle = Box::new(circle) as Box<dyn Circle>;
    let nonsense = circle.radius() * circle.area();

    println!("{}",nonsense);

}

#![allow(dead_code, unused_variables)]
/// easyrust-closure-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p easyrust-closure_bin --bin easyrust-closure-ex-1```
///
/// ```cargo doc  --package easyrust-closure_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package easyrust-closure_bin ```
///
/// ## What
/// `TODO`
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
fn returns_a_closure(input: &str) -> impl FnMut(i32) -> i32 {
    match input {
        "double" => |mut number| {
            number *= 2;
            println!("Doubling number. Now it is {}", number);
            number
        },
        "triple" => |mut number| {
            number *= 40;
            println!("Tripling number. Now it is {}", number);
            number
        },
        _ => |number| {
            println!("Sorry, it's the same: {}.", number);
            number
        },
    }
}
fn main() {
    let my_number = 10;
    let mut doubles = returns_a_closure("double");
    let mut triples = returns_a_closure("triple");
    let mut quadruples = returns_a_closure("quadruple");
    doubles(my_number);
    triples(my_number);
    quadruples(my_number);
}

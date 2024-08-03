#![allow(dead_code, unused_variables)]
/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-doc-lifetime-generic_bin --bin rust-doc-lifetime-generic-main```
///
/// ```cargo doc  --package rust-doc-lifetime-generic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-lifetime-generic_bin```
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
/// `30`
///
/// ## Example
/// `TODO`
/// Here, Rust infers a lifetime that is as short as possible.
/// The two references are then coerced to that lifetime.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

/// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
/// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
fn choose_first<'a: 'b, 'b>(first: &'a i32, second: &'b i32) -> &'b i32 {
    second
}

fn main() {
    let first = 2; // Longer lifetime

    {
        let second = 3; // Shorter lifetime

        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
}

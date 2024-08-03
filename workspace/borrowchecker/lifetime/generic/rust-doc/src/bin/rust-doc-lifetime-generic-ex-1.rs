#![allow(dead_code, unused_variables)]
/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-doc-lifetime-generic_bin --bin rust-doc-lifetime-generic-ex-1```
///
/// ```cargo doc  --package rust-doc-lifetime-generic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-lifetime-generic_bin```
///
/// ## What
/// `Bound`
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

use std::fmt::Debug; // Trait to bound with.

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
/// `Ref` contains a reference to a generic type `T` that has
/// an unknown lifetime `'a`. `T` is bounded such that any
/// *references* in `T` must outlive `'a`. Additionally, the lifetime
/// of `Ref` may not exceed `'a`.
/// A generic function which prints using the `Debug` trait.
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

/// Here a reference to `T` is taken where `T` implements
/// `Debug` and all *references* in `T` outlive `'a`. In
/// addition, `'a` must outlive the function.
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}

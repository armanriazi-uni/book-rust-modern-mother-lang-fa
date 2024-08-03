#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-customized-box_bin --bin rust-doc-customized-box-main```
///
/// ```cargo doc  --package rust-doc-customized-box_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-customized-box_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
/// `Deref trait to override the * operator`
/// Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.
/// Rust does deref coeboxion when it finds types and trait implementations in three cases:
/// From &T to &U when T: Deref<Target=U>
/// From &mut T to &mut U when T: DerefMut<Target=U>
/// From &mut T to &U when T: Deref<Target=U>

/// `&(*m)[..])`
/// The (*m) dereferences the MyBox<String> into a String. Then the & and [..] take a string slice of the String that is equal to the whole string to match the signature of hello. This code without deref coercions is harder to read, write, and understand with all of these symbols involved. Deref coercion allows Rust to handle these conversions for us automatically.
/// When the Deref trait is defined for the types involved, Rust will analyze the types and use Deref::deref as many times as necessary to get a reference to match the parameter’s type. 
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
/// //rust,compile_fail,no_run,ignore
///  `TODO`



use std::ops::{Deref, DerefMut};

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T>
{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn print_hello(name: &str) {
    println!("Hello, {}!", name);
}
//implementation of Deref on String that returns a string slice, and this is in the API documentation for Deref. Rust calls deref again to turn the &String into &str, which matches the hello function’s definition.
fn main() {
    let m = MyBox::new(String::from("Rust"));
    print_hello(&(*m)[..]);
    //
    let x=5;
    let y = MyBox::new(x);
    let z= y.deref();
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
}

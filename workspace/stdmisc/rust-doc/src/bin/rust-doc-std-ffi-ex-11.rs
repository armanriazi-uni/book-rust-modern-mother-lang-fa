#![allow(dead_code, unused_variables)]



/// rust-doc-std-ffi-ex-11.rs
/// cli with clap
/// https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html
/// 
/// ## Commands
///
/// ```cargo run -q -p rust_doc_std_bin  --bin rust-doc-std-ffi-ex-11```
///
/// ```cargo doc  --package rust_doc_std_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust_doc_std_bin ```
///
/// ## What
/// `Arg command line` #ffi
///
/// ## How
/// Rust provides a Foreign Function Interface (#FFI) to C libraries. Foreign functions must be declared inside an extern block annotated with a #[link] attribute containing the name of the foreign library.
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// 
/// Rust provides a Foreign Function Interface (FFI) to C libraries. Foreign functions must be declared inside an extern block annotated with a #[link] attribute containing the name of the foreign library.
/// 
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// //```compile_fail,ignore

use std::fmt;

// this extern block links to the libm library
#[link(name = "m")]
extern {
    // this is a foreign function
    // that computes the square root of a single precision complex number
    fn csqrtf(z: Complex) -> Complex;

    fn ccosf(z: Complex) -> Complex;
}

// Since calling foreign functions is considered unsafe,
// it's common to write safe wrappers around them.
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

fn main() {
    // z = -1 + 0i
    let z = Complex { re: -1., im: 0. };

    // calling a foreign function is an unsafe operation
    let z_sqrt = unsafe { csqrtf(z) };

    println!("the square root of {:?} is {:?}", z, z_sqrt);

    // calling safe API wrapped around unsafe operation
    println!("cos({:?}) = {:?}", z, cos(z));
}

// Minimal implementation of single precision complex numbers
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
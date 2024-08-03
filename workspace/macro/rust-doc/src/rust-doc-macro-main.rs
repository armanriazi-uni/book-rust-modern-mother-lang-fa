#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-doc-macro_bin --bin rust-doc-macro-main```
///
/// ```cargo doc  --package rust-doc-macro_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-macro_bin ```
///
/// ## What
/// `macro-share behavior`
///
/// ## How
/// The macro above takes a list of parameters in $( $x:expr ),*, then it uses them in a loop to sum all the costs of the products provided.
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// ```rust

macro_rules! create_method {
    ($vis:vis $name:ident) => {
        $vis fn $name(&self) {}
    };
}

trait T1 {
    // Empty `vis` is allowed.
    create_method! { method_of_t1 }
}

struct S;

impl S {
    // Visibility is allowed here.
    create_method! { pub method_of_s }
}

impl T1 for S {}

fn main() {
    let s = S;
    s.method_of_t1();
    s.method_of_s();
}

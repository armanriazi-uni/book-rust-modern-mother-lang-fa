#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-types-static_bin --bin master-rust-types-static-main```
///
/// ```cargo egg  --package master-rust-types-static_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --egg  --package master-rust-types-static_bin```
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
/// 'nothing'
///
/// ## Example
/// //```rust,compile_fail,ignore
static mut BAZ: u32 = 4;
static FOO: u8 = 9;

fn main() {
    unsafe {
        println!("baz is {}", BAZ);
        BAZ = 42;
        println!("baz is now {}", BAZ);
        println!("foo is {}", FOO);
    }
}

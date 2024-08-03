#![allow(dead_code, unused_variables)]
extern crate num_cpus;
/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-doc-cpu_bin --bin rust-doc-cpu-main```
///
/// ```cargo doc  --package rust-doc-cpu_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-cpu_bin ```
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
///
fn main() {
    println!("There are {} CPUs", num_cpus::get());
}

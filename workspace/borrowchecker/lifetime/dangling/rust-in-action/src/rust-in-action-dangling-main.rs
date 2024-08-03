#![allow(dead_code, unused_variables)]
/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-in-action-dangle_bin --bin rust-in-action-dangling-main```
///
/// ```cargo doc  --package rust-in-action-dangle_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-dangle_bin```
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
/// ```compile_fail,ignore
/// fn main() {
/// let mut grains: Vec<Cereal> = vec![];
/// grains.push(Cereal::Rye);
/// drop(grains);
/// println!("{:?}", grains); //^^^^^^ value borrowed here after move
/// ```

fn main() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    drop(grains);
}
#[derive(Debug)]
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

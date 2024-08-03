/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-dangle_bin --bin rust-egg-dangling-main```
///
/// ```cargo doc  --package rust-egg-dangle_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-dangle_bin```
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


        //let reference_to_nothing = dangle();
        let reference_to_nothing = no_dangle();

}
// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
    //Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

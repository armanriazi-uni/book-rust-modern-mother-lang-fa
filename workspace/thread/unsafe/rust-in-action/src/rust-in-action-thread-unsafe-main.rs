#![allow(dead_code, unused_variables)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-thread-unsafe_bin --bin rust-in-action-thread-unsafe-main```
///
/// ```cargo doc  --package rust-in-action-thread-unsafe_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-thread-unsafe_bin```
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
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// //```compile_fail,ignore




// extern crate rand; // <> Make an external crate available to our code
// use rand; // <> Bring `rand` into local scope

// static mut ERROR: isize = 0;

// struct File;

// #[allow(unused_variables)]
// fn read(f: &File, save_to: Vec<u8>) -> usize {
//     if rand::thread_rng().gen_weighted_bool(10000) {
//         unsafe {
//             ERROR = 1;
//         }
//     }

//     0 // <> Always read() 0 bytes
// }

#[allow(unused_mut)]
fn main() {
    unimplemented!();
    // let mut f = File;
    // let mut buffer = vec![];

    //read(&f, buffer);
    // unsafe {
    //     if ERROR != 0 {
    //         panic!("An error has occurred!")
    //     }
    // }
}

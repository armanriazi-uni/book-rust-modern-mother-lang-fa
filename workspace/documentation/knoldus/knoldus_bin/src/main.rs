#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://doc.rust-lang.org/"
)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]

use knoldus_lib::core::cube::calculate_cube_of_integer;
use knoldus_lib::core::square::calculate_square_of_integer;
// Example from the futures-rs library

/// ### How does it run?
/// ```cargo doc  --workspace --message-format short --no-deps --open --color always```
///
/// ### Commands
/// 
/// ```cargo run -p knoldus_bin -q```
/// ### What it does
/// Checks for ... (describe what the lint matches).
///
/// ### How it does
/// Supply the reason for linting the code.

/// This application is intended to calculate cube and square of integer values
/// # Depricated
/// ```rust
/// #[cfg_attr(tarpaulin, ignore)]
///```
/// #[cfg(not(tarpaulin_include))]
// #[macro_use] extern crate doctest
fn main() {
    let number: u32 = 5;
    println!(
        "Square of {} is {}",
        number,
        calculate_square_of_integer(number)
    );
    println!(
        "Cube of {} is {}",
        number,
        calculate_cube_of_integer(number)
    );
}

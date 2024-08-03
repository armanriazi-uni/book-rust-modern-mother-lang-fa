#![allow(dead_code, unused_variables)]
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p mod_draw_bin --bin mod-draw-main```
///
/// ```cargo doc  --package mod_draw_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package mod_draw_bin ```
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


use rgb::*;
//extern crate mod_draw_lib;
/// https://doc.rust-lang.org/cargo/reference/features.html

fn main() {
    println!("Hello World");
    let px = RGB {
        r: 255_u16,
        g: 0_u16,
        b: 255_u16,
    };
    let width = 10;
    let height = 20;

    let rc = mod_draw_lib::core::mod_shapes::Rectangle {
        color: px,
        width,
        height,
    };

    mod_draw_lib::core::mod_color::draw_line(
        rc.width.try_into().unwrap(),
        rc.height.try_into().unwrap(),
        &rc.color,
    );
}

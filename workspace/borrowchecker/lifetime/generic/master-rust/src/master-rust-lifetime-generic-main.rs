#![allow(dead_code, unused_variables)]
/// Main
///
/// # Commands
///
/// ```cargo run -q -p master-rust-lifetime-generic_bin --bin master-rust-lifetime-generic-main```
///
/// ```cargo doc  --package master-rust-lifetime-generic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-lifetime-generic_bin```
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
/// `30`
///
/// ## Example
/// `TODO`

#[derive(Debug)]
struct Number<'a> {
    num: &'a u8
}

impl<'a> Number<'a> {
    fn get_num(&self) -> &'a u8 {
        self.num
    }
    fn set_num(&mut self, new_number: &'a u8) {
        self.num = new_number
    }
}

fn main() {
    let a = 10;
    let mut num = Number { num: &a };
    num.set_num(&23);
    println!("{:?}", num.get_num());
}

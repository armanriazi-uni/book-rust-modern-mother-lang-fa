#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-unsafe-send-sync_bin --bin rust-doc-unsafe-send-sync-ex-1```
///
/// ```cargo doc  --package rust-doc-unsafe-send-sync_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-unsafe-send-sync_bin```
///
/// ## What
// `TODO`
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

trait MyTrait {
    fn do_something(&self);
}

struct MyStruct {
    value: i32,
}

// unsafe impl !Send for MyStruct {}
// unsafe impl !Sync for MyStruct {}

impl MyTrait for MyStruct {
    fn do_something(&self) {
        println!("Doing something with {}", self.value);
    }
}

fn main() {
    let my_struct = MyStruct { value: 42 };
    my_struct.do_something();
}

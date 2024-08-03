#![allow(dead_code, unused_variables)]

/// rust-doc-smartpointer-arc-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-smartpointer-arc_bin --bin  rust-doc-smartpointer-arc-ex-3```
///
/// ```cargo doc  --package rust-doc-smartpointer-arc_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-smartpointer-arc_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `Nightly needs`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `unimplemented`
///
/// ## Example
/// `TODO`


//#![feature(new_uninit)]
//#![feature(get_mut_unchecked)]
//use std::sync::Arc;

//fn main(){
    //let mut five = Arc::<u32>::new_uninit();

    //// Deferred initialization:
    //Arc::get_mut(&mut five).unwrap().write(5);

    //let five = unsafe { five.assume_init() };

    //assert_eq!(*five, 5)
//}

use std::sync::Arc;
use std::cell::RefCell;
fn main() {
    // Directly initialize an Arc<u32> with a value of 5
    let five = Arc::new(RefCell::new(5));

    // Safely access and modify the value inside the RefCell
    *five.borrow_mut() = 10; // Change the value to 10

    // Deref to get the inner value
    let inner_value = *five.borrow();
    println!("The new value is {}", inner_value);

    // Assert that the value has been changed
    assert_eq!(inner_value, 10);
}

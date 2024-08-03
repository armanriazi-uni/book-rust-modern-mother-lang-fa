#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-rc_bin --bin rust-doc-rc-ex-4```
///
/// ```cargo doc  --package rust-doc-rc_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-rc_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
///  `TODO`
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
/// //rust,compile_fail,no_run,ignore
///  `TODO`
///
use std::rc::{Rc, Weak};
fn main(){
    let strong = Rc::new("hello".to_owned());
    let raw_1 = Rc::downgrade(&strong).into_raw();
    let raw_2 = Rc::downgrade(&strong).into_raw();
    assert_eq!(2, Rc::weak_count(&strong));
    assert_eq!("hello", &*unsafe { Weak::from_raw(raw_1) }.upgrade().unwrap());
    assert_eq!(1, Rc::weak_count(&strong));
    drop(strong);
    // Decrement the last weak count.
    assert!(unsafe { Weak::from_raw(raw_2) }.upgrade().is_none());
}

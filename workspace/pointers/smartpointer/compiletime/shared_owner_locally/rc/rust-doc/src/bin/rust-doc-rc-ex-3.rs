#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-rc_bin --bin rust-doc-rc-ex-3```
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
use std::rc::Rc;
use std::ptr;
fn main(){
    let strong = Rc::new("hello".to_owned());
    let weak = Rc::downgrade(&strong);
    // Both point to the same object
    assert!(ptr::eq(&*strong, weak.as_ptr()));
    // The strong here keeps it alive, so we can still access the object.
    assert_eq!("hello", unsafe { &*weak.as_ptr() });
    drop(strong);
    // But not any more. We can do weak.as_ptr(), but accessing the pointer would lead to
    // undefined behaviour.
    // assert!(ptr::eq(&*strong, weak.as_ptr()));
    // assert_eq!("hello", unsafe { &*weak.as_ptr() });
}

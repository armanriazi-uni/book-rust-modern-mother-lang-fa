#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-rc_bin --bin rust-doc-rc-ex-6```
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
fn main(){
    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    assert!(strong_five.is_some());
    assert!(weak_five.upgrade().is_none());
    drop(strong_five);
    drop(five);
}

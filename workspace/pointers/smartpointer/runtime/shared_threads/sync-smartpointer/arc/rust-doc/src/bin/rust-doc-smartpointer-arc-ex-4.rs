#![allow(dead_code, unused_variables)]


/// rust-doc-smartpointer-arc-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-smartpointer-arc_bin --bin  rust-doc-smartpointer-arc-ex-4```
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

//#![feature(collections)]
//#![feature(new_uninit)]
//use std::sync::Arc;

////let mut zero = Arc::<u32>::new_uninit();
//let zero = Arc::<u32>::try_new_zeroed().unwrap();
//let  zero = unsafe {zero.assume_init() };

//assert_eq!(zero, 0)
//}
// #![feature(new_uninit, allocator_api)]

// use std::sync::Arc;

// fn main(){
// let zero = Arc::<u32>::try_new_zeroed()?;
// let zero = unsafe { zero.assume_init() };

// assert_eq!(*zero, 0);
// }
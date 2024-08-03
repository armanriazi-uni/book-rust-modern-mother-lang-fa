#![allow(dead_code, unused_variables)]
/// code-like-pro-allocator-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p code-like-pro-allocator_bin --bin code-like-pro-allocator-ex-2```
///
/// ```cargo doc  --package code-like-pro-allocator_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package code-like-pro-allocator_bin```
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
/// `nothig`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// ```compile_fail,ignore
// #![feature(allocator_api)]

// use std::alloc::{AllocError, Allocator, Global, Layout};
// use std::ptr::NonNull;

// pub struct PassThruAllocator;

// unsafe impl Allocator for PassThruAllocator {
//     fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
//         Global.allocate(layout)
//     }
//     unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
//         Global.deallocate(ptr, layout)
//     }
// }

fn main() {
    // let mut custom_alloc_vec: Vec<i32, _> = Vec::with_capacity_in(10, PassThruAllocator);
    // for i in 0..10 {
    //     custom_alloc_vec.push(i as i32 + 1);
    // }
   //println!("custom_alloc_vec={:?}", custom_alloc_vec);
   unimplemented!();
}

#![allow(dead_code, unused_variables)]



/// Main
///
/// ## Commands
///
/// ```cargo run -q -p code-like-pro-allocator_bin --bin code-like-pro-allocator-main```
///
/// ```cargo doc  --package code-like-pro-allocator_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package code-like-pro-allocator_bin```
///
/// ## What
/// `BasicAllocator`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `custom_alloc_vec=[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// ```compile_fail,ignore
#![feature(allocator_api)]

use std::alloc::{AllocError, Allocator, Layout};
use std::ptr::NonNull;
use libc::{free, malloc};
pub struct BasicAllocator;
unsafe impl Allocator for BasicAllocator {
    // Allocate memory using `malloc`, then wrap it in a slice
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = unsafe { malloc(layout.size() as libc::size_t) };
        if ptr.is_null() {
            return Err(AllocError);
        }
        let slice = std::slice::from_raw_parts_mut(ptr as *mut u8, layout.size());
        Ok(NonNull::new_unchecked(slice))
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        free(ptr.as_ptr() as *mut libc::c_void);
    }
}

fn main() {    
    let mut custom_alloc_vec: Vec<i32, BasicAllocator> = Vec::with_capacity(10);
    for i in 0..10 {
        custom_alloc_vec.push(i as i32 + 1);
    }
    println!("custom_alloc_vec={:?}", custom_alloc_vec);
}

/// other-mem-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p other-mem_bin --bin  other-mem-ex-2```
///
/// ## What
/// `Printing from strings provided by external sources`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return

use std::ptr;

struct Block {
    size: usize,
    next: *mut Block,
}

unsafe impl Drop for Block {
    fn drop(&mut self) {
        unsafe { ptr::drop_in_place(self); }
    }
}

struct Allocator {
    free_list: *mut Block,
}

impl Allocator {
    fn new() -> Self {
        let block = Box::new(Block {
            size: 4096, // Initial block size
            next: ptr::null_mut(),
        });
        let block_ptr = Box::into_raw(block);
        let mut allocator = Allocator { free_list: block_ptr };
        allocator.init();
        allocator
    }

    fn init(&mut self) {
        let block_size = std::mem::size_of::<Block>() + 4096; // Adjusted to include Block size and additional data
        let block = ptr::alloc(block_size) as *mut Block;
        block.as_mut().unwrap().size = block_size;
        block.as_mut().unwrap().next = self.free_list;
        self.free_list = block;
    }

    fn allocate(&mut self, size: usize) -> Option<&[u8]> { // Changed return type to &[u8]
        let current = self.free_list;
        loop {
            let block = unsafe { &*current };
            if block.size >= size {
                let result = block as *mut u8; // Now correctly aligned with the expected return type
                let remaining_size = block.size - size;
                if remaining_size > 0 {
                    // Split the block
                    let split_block = ptr::alloc(remaining_size) as *mut Block;
                    split_block.as_mut().unwrap().size = remaining_size;
                    split_block.as_mut().unwrap().next = block.next;
                    unsafe {
                        block.next = split_block;
                    }
                } else {
                    // No remaining space, just move the pointer
                    block.next = block.next.offset(remaining_size as isize);
                }
                return Some(std::slice::from_raw_parts(result, size)); // Convert raw pointer to slice
            }
            current = block.next;
            if current.is_null() {
                return None; // No more free blocks
            }
        }
    }

    fn deallocate(&mut self, ptr: *mut u8, size: usize) {
        let block = unsafe { &mut *(ptr as *mut Block) };
        block.size = size;
        block.next = self.free_list;
        self.free_list = ptr as *mut Block;
        unsafe { ptr::drop_in_place(ptr as *mut Block); }
    }
}

fn main() {
    // Create a new allocator
    let mut allocator = Allocator::new();

    // Allocate memory for 10 integers
    let int_slice = allocator.allocate(40).expect("Failed to allocate memory");
    println!("Allocated memory at {:p}", int_slice);

    // Write values to the allocated memory
    for i in 0..10 {
        unsafe {
            *(int_slice.get_unchecked(i) as *mut i32) = i as i32;
        }
    }

    // Deallocate the memory
    allocator.deallocate(int_slice.as_ptr(), 40);

    println!("Memory successfully deallocated.");
}

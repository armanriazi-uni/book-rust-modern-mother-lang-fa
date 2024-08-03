/// other-mem
///
/// ## Commands
///
/// ```cargo test -q -p other-mem_bin --bin  other-mem-ex-1```
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
// Add necessary imports at the top
use std::mem::{align_of, size_of};
use std::os::raw::c_void;

// Define a function to get the page size
fn get_page_size() -> usize {
    #[cfg(target_os = "windows")]
    {
        const PAGE_SIZE: usize = 4096; // Common page size on Windows
        PAGE_SIZE
    }
    #[cfg(target_os = "linux")]
    {
        unsafe { sysconf(_SC_PAGESIZE) } // Linux uses sysconf to get page size
    }
    #[cfg(any(target_os = "macos", target_os = "freebsd"))]
    {
        4096 // Common page size on macOS and FreeBSD
    }
    else {
        panic!("Unsupported OS");
    }
}

// Modify the struct and methods to use `get_page_size`
struct SimpleAllocator {}

unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let aligned_layout = match layout.align_to(max(layout.align(), get_page_size())) {
            Ok(l) => l.pad_to_align(),
            Err(_) => return ptr::null_mut(),
        };

        #[cfg(windows)]
        {
            let addr = libc::VirtualAlloc(
                ptr::null_mut(),
                aligned_layout.size(),
                libc::MEM_COMMIT | libc::MEM_RESERVE,
                libc::PAGE_READWRITE,
            );
            addr as _
        }
        #[cfg(unix)]
        {
            let addr = libc::mmap(
                ptr::null_mut(),
                aligned_layout.size(),
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0,
            );
            addr as _
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Ok(aligned) = layout.align_to(max(layout.align(), get_page_size())) {
            #[cfg(windows)]
            libc::VirtualFree(ptr as _, aligned.pad_to_align().size(), libc::MEM_RELEASE);
            #[cfg(not(windows))]
            libc::munmap(ptr as _, aligned.pad_to_align().size());
        }
    }
}

#[global_allocator]
static ALLOCATOR: SimpleAllocator = SimpleAllocator {};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("page size is {}", get_page_size());
        let x = Box::new(99);
        println!("x={}", x);
    }
}

fn main(){
    unimplemented!();
}

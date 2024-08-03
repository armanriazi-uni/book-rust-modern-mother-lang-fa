#![allow(dead_code, unused_variables)]



/// code-like-pro-allocator-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p code-like-pro-allocator_bin --bin code-like-pro-allocator-ex-3```
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
#![feature(allocator_api)]

use lazy_static::lazy_static;
use std::alloc::{AllocError, Allocator, Layout};
use std::ptr;
use std::io::Error;
use std::mem::size_of_val;

// Define a trait for platform-specific memory protection
trait MemoryProtector {
    fn protect(&self, data: &[u8], prot: u32) -> Result<(), Error>;
}

#[cfg(unix)]
impl MemoryProtector for () {
    fn protect(&self, data: &[u8], prot: u32) -> Result<(), Error> {
        use libc::{mprotect as c_mprotect, c_void};

        let ret = unsafe { c_mprotect(data.as_ptr() as *mut c_void, data.len(), prot) };
        match ret {
            0 => Ok(()),
            _ => Err(Error::last_os_error()),
        }
    }
}

#[cfg(windows)]
impl MemoryProtector for () {
    fn protect(&self, data: &[u8], prot: u32) -> Result<(), Error> {
        use winapi::shared::minwindef::{DWORD, LPVOID};
        use winapi::um::memoryapi::VirtualProtect;
        use winapi::um::winnt::{PAGE_EXECUTE_READWRITE, PAGE_EXECUTE_READ, PAGE_EXECUTE_WRITECOPY};

        let mut old: DWORD = 0;

        let res = unsafe {
            VirtualProtect(
                data.as_ptr() as LPVOID,
                data.len(),
                match prot {
                    0b00000001 => PAGE_EXECUTE_READ,
                    0b00000010 => PAGE_EXECUTE_WRITECOPY,
                    0b00000100 => PAGE_EXECUTE_READWRITE,
                    _ => panic!("Invalid protection flag"),
                },
                &mut old,
            )
        };
        match res {
            1 => Ok(()),
            _ => Err(Error::last_os_error()),
        }
    }
}

struct PageAlignedAllocator;

lazy_static! {
    static ref PAGESIZE: usize = {
        #[cfg(unix)]
        {
            use libc::{sysconf, _SC_PAGE_SIZE};
            unsafe { sysconf(_SC_PAGE_SIZE) as usize }
        }
        #[cfg(windows)]
        {
            use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
            let mut si = SYSTEM_INFO::default();
            unsafe { GetSystemInfo(&mut si) };
            si.dwPageSize as usize
        }
    };
}

unsafe impl Allocator for PageAlignedAllocator {
    fn allocate(&self, layout: Layout) -> Result<ptr::NonNull<[u8]>, AllocError> {
        let pagesize = *PAGESIZE;
        let size = (layout.size().align_to(page_size)[1] + pagesize - 1) / pagesize * pagesize; // Align to page size
        let out = match std::alloc::alloc(size) {
            Some(out) => out,
            None => return Err(AllocError),
        };

        // Protect the memory
        let protector = ();
        protector.protect(&out[..], 0b00000100).map_err(|_| AllocError)?; // PAGE_EXECUTE_READWRITE

        Ok(ptr::NonNull::new_unchecked(out))
    }

    unsafe fn deallocate(&self, ptr: ptr::NonNull<u8>, layout: Layout) {
        let protector = ();
        protector.protect(&*ptr, 0).map_err(|_| ())?; // Reset to default protection

        std::alloc::dealloc(ptr.as_ptr(), layout);
    }
}

fn main() {
    let mut custom_alloc_vec: Vec<i32, _> = Vec::with_capacity_in(10, PageAlignedAllocator);
    for i in 0..10 {
        custom_alloc_vec.push(i as i32 + 1);
    }
    println!("custom_alloc_vec={:?}", custom_alloc_vec);
}

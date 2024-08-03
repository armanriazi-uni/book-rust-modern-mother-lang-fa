#![allow(dead_code, unused_variables)]

/// code-like-pro-libc-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p code-like-pro-libc_bin --bin  code-like-pro-libc-ex-2```
///
/// ## What
/// `macro libc`
///
/// ## How
/// We can unwrap the result value, which will return the contained value. However, it’ll panic if it’s an macro. We recommend not using unwrap unless you need the code to panic (like in testing).
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `unimplemented`
///
/// ## Example
/// //```rust,no_run,compile_fail,ignore
use libc::{c_char, c_int, c_uchar, c_uint};
use std::ffi::CString;

#[repr(C)]
struct GzFileState {
    have: c_uint,
    next: *mut c_uchar,
    pos: i64,
}

type GzFile = *mut GzFileState;

#[link(name = "z")]
extern "C" {
    fn gzopen(path: *const c_char, mode: *const c_char) -> GzFile;
    fn gzread(file: GzFile, buf: *mut c_uchar, len: c_uint) -> c_int;
    fn gzclose(file: GzFile) -> c_int;
    fn gzeof(file: GzFile) -> c_int;
}

fn read_gz_file(name: &str) -> String {
    let mut buffer = [0u8; 0x1000];
    let mut contents = String::new();
    unsafe {
        let c_name = CString::new(name).expect("CString failed");
        let c_mode = CString::new("r").expect("CString failed");
        let file = gzopen(c_name.as_ptr(), c_mode.as_ptr());
        if file.is_null() {
            panic!("Couldn't read file: {}", std::io::Error::last_os_error());
        }
        while gzeof(file) == 0 {
            let bytes_read = gzread(file, buffer.as_mut_ptr(), (buffer.len() - 1) as c_uint);
            let s = std::str::from_utf8(&buffer[..(bytes_read as usize)]).unwrap();
            contents.push_str(s);
        }
        gzclose(file);
    }

    contents
}

fn main() {
    println!("{}", read_gz_file("src/main.rs.gz"));
}

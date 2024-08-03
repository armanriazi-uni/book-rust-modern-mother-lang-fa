#![allow(dead_code, unused_variables)]
use std::slice;
use std::str;

/// rust-doc-dangling-bin-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-dangle_bin --bin  rust-doc-dangling-bin-ex-3```
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
/// `unimplemented`
///
/// ## Example
/// ```rust,no_run,compile_fail,ignore
///```
///
fn main() {
    let story = "Once upon a time...";
    let ptr = story.as_ptr();
    let len = story.len();
    assert_eq!(19, len);
    let s = unsafe {        
        let slice = slice::from_raw_parts(ptr, len);

        // ... and then convert that slice into a string slice
        str::from_utf8(slice)
    };
    assert_eq!(s, Ok(story));
}

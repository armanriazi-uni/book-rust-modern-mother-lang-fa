#![allow(dead_code, unused_variables)]
/// ultimate-ffi-main
///
/// ## Commands
/// 
/// ```rustc -L . -o ultimate-ffi-main ./workspace/unsafe/extern-ffi/ultimate/src/ultimate-ffi-main.rs -l ultimate-ffi_bin```
///
/// ```cargo run -q -p ultimate-ffi_bin --bin ultimate-ffi-main```
///
/// ```cargo doc  --package ultimate-ffi_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package ultimate-ffi_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// $ gcc -c external_lib.c -o external_lib.o
/// $ ar rcs libexternal_lib.a external_lib.o
/// $ rustc -L . -o main main.rs -l external_lib
/// $ ./main
/// Output: 1 2 3 4 5
///
/// ## Example
/// //```rust,compile_fail,ignore
///
//#[no_mangle]

extern "C" {
    fn process_data(data: *mut u8, length: usize);
}
fn main() {
    let mut data: Vec<u8> = vec![1, 2, 3, 4, 5];
    unsafe {
        process_data(data.as_mut_ptr(), data.len());
    }
}

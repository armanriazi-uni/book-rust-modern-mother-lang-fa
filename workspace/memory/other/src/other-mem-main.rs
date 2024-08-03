/// other-mem
///
/// ## Commands
///
/// ```cargo run -q -p other-mem_bin --bin  other-mem-main```
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
fn main() {
    let mut x = 0;
    let y = &mut x as *mut i32;
    let z = 12;
    unsafe {
        std::ptr::write(y, z);
        assert_eq!(std::ptr::read(y), 12);
    }
}
    
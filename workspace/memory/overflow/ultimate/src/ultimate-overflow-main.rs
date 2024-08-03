/// ultimate-overflow
///
/// ## Commands
///
/// ```cargo run -q -p ultimate-overflow_bin --bin  ultimate-overflow-main```
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
/// 



fn main() {
    let mut buffer: [u8; 5] = [0; 5]; // ①
    let data = b"Overflowing Content!"; // ②
    buffer[..data.len()].copy_from_slice(data); // ③
    println!("Buffer: {:?}", buffer); // ④
}
    
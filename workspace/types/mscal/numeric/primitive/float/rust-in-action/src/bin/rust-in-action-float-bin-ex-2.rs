#![allow(dead_code, unused_variables)]

/// rust-in-action-float-bin-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-float_bin --bin rust-in-action-float-bin-ex-2```
///
/// ## What
/// Q1_7 - single byte representation of a fixed point number with range [-1, 1].
/// The name refers to the Texas Instrument representation
///
/// ## References:
///  - English Wikipedia: "Q (number format)" https://en.wikipedia.org/wiki/Q_(number_format)
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `assert:true`
///
/// ## Example
/// ```rust,compile_fail,ignore
/// use num::{Float};
/// struct Q1_7(i8); // tuple struct holding a i8 value
///
/// impl From<T: Float> for Q1_7 {
///    fn from (n:T) -> Self {
///        let val = if n > 1.0 { // out of bounds numbers are coerced to the maximum of the range
///            1
///        } else if n < -1.0 {
///            -1
///        } else {
///            n * (2**7)
///        }
///
///        Q1_7(val as i8)
///    }
///}
///
///impl From for U: Float {
///    fn from(q: Q1_7) -> U {
///        q.0 * (2 ** -7)
///    }
///}
///
///mod tests {
///    use super::*;
///
///    test
///}
///
/// ```

fn main() {
    unimplemented!()
}

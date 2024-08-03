#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-float_bin --bin rust-in-action-types-float-main```
///
/// ```cargo doc  --package rust-in-action-types-float_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-types-float_bin```
///
/// ## What
/// Rust includes some tolerances to allow comparisons between floating-point values.
/// These tolerances are defined as f32::EPSILON and f64::EPSILON. To be more precise,
/// it’s possible to get closer to how Rust is behaving under the hood, as the following small example shows.
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// EPSION=`0.00000011920929`
///
/// ## Example
/// ```rust,compile_fail,ignore
/// fn main() {
/// assert!(0.1 + 0.2 == 0.3);
/// }
/// ```
///
/// ```rust,compile_fail,ignore
/// fn main() {
///  let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
///  let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

///  println!("abc (f32)");
///  println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
///  println!("         0.3: {:x}", (abc.2).to_bits());
///  println!();

///  println!("xyz (f64)");
///  println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
///  println!("         0.3: {:x}", (xyz.2).to_bits());
///  println!();

///  assert!(abc.0 + abc.1 == abc.2);
///  assert!(xyz.0 + xyz.1 == xyz.2);
/// }
/// ```

/// ```rust,compile_fail,ignore
/// fn main() {
///     let x = (-42.0_f32).sqrt(); //-42.0 is NAN so NAN values are never equal
///     assert_eq!(x, x);
///   }
/// ```

fn main() {
    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let absolute_difference = (desired - result).abs();
    assert_eq!(absolute_difference, 0.0);
    println!("{}",f32::EPSILON);
    assert!(absolute_difference <= f32::EPSILON);
}

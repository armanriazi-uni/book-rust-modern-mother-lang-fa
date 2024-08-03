#![allow(dead_code, unused_variables)]

/// rust-in-action-float-bin-ex-5
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-float_bin --bin rust-in-action-float-bin-ex-5```
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
/// `abc (f32)
///   0.1 + 0.2: 3e99999a
///         0.3: 3e99999a
///
///xyz (f64)
///   0.1 + 0.2: 3fd3333333333334
///         0.3: 3fd3333333333333`
///
/// ## Example
/// //```rust,compile_fail,ignore

fn main() {
     let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
     let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

     println!("abc (f32)");
     println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
     println!("         0.3: {:x}", (abc.2).to_bits());
     println!();

     println!("xyz (f64)");
     println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
     println!("         0.3: {:x}", (xyz.2).to_bits());
     println!();

     assert!(abc.0 + abc.1 == abc.2);
     assert_ne!(xyz.0 + xyz.1 , xyz.2);
 }

#![allow(dead_code, unused_variables)]

use std::result;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p other-math-max_bin --bin other-math-is-gcd-ex3```
///
/// ## What
/// `GCD - Greatest common divisor`
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
///```rust


fn GCD(a:i32, b:i32)-> i32{
   if a == 0{
       return b;
   }
   else if a == 1{
       return 1;
   }
   else{
       return GCD(b%a, a);
   }
}

fn main(){
let a = 48;
let b = 72;

if a > b{
  println!("invalid input, a must be <= to b");
  }
else{
  println!("GCD of  a and  b  = {:?}", GCD(a,b));
}

}

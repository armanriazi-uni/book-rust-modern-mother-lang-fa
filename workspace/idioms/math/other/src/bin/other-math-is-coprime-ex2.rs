#![allow(dead_code, unused_variables)]

use std::result;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p other-math-max_bin --bin other-math-is-coprime-ex2```
///
/// ## What
/// `GCD - coprime`
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

/// GCD : returns returns GCD of a and b.
/// parameters a and b
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
    0
}

/// isCoPrime : returns 1 when a and b are coprime to each other,
/// otherwise returns 0
/// parameters a and b
fn isCoPrime(a:i32, b:i32)-> i32{
   if a > b{
    if GCD(b, a)==1{
        return  1;
    }
    else {
        return 0;
    }
   }
   else{
        if GCD(a, b)==1{
            return 1;
        }
        else{
            return 0;
        }
    }
    0
}
fn main(){
let a = 18;
let b = 5;
let mut result="";
if isCoPrime(a, b)== 1 {
 result="coprime";
}
else{
 result="not coprime";
}
println!("{}", result);
}

#![allow(dead_code, unused_variables)]

/// rust-egg-return-bin-ex-5
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-return_bin --bin rust-egg-return-bin-ex-5```
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
/// `Printed:54`
/// ## Example
/// //```rust,no_run
fn main(){
    let mut res=42;
    let option = Some(12);
    if let Some(x)=option{
        res+=x;
    }
    /* no clippy code
    for x in option{
        res+=x;
    }*/
    println!("Printed:{}",res);

}

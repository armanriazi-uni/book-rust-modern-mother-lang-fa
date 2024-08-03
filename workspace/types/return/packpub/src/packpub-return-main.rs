#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p packpub-return_bin --bin packpub-return-main```
///
/// ```cargo doc  --package packpub-return_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package packpub-return_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your return/func name]
///
/// # Return
/// `NoString\n Errrorr - Not enough Args`
///
/// ## Example
/// //```rust,compile_fail,ignore
use std::collections::HashMap;
use std::env::args;



// cargo run  arg1 arg2=sth arg3


//args:usize
fn main() {

    let mut hm = HashMap::new();

    hm.insert(3 ,"Hello");
    hm.insert(5, "world");

    let r = hm.get(&4).unwrap();//&"NoString"

    println!("{}", r);

    match get_arg(3) {
        Ok(v)=>println!("OK - {}", v),
        Err(e)=>println!("Error - {}",e),
    }
}


fn get_arg(n:usize)->Result<String,String>{
    //yields pairs (i, val)
    for (i,a) in args().enumerate(){
        if i == n{
            return Ok(a);
        }
    }
    Err("Not enough Args".to_string())
}

#![allow(dead_code, unused_variables)]

use std::io::prelude::*;
use std::{fs::File, io::Read};
use rand::{random};
static mut ERROR: isize = 0;



/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo test -q -p packtpub-env_bin --bin packtpub-env-main```
/// 
///```RUST_BACKTRACE=full cargo test -q -p packtpub-env_bin --bin packtpub-env-main -- - ROAD Route66```
///
/// ```cargo doc  --package packtpub-env_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package packtpub-env_bin ```
///
/// ## What
/// `error-conversion`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///
///

use std::env::{var,set_var};

pub fn road_len()->usize{
    let e = var("ROAD").unwrap_or("".to_string());
    e.len()
}

pub fn rail_len()->usize{
    let s = var("GWR").unwrap_or("".to_string());
    _rail_len(&s)
}

fn _rail_len(s:&str)->usize{
    s.len()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        //let e = var("HELLO").unwrap();
        //assert_eq!(&e,"WORLD");

        set_var("ROAD", "Route66");//can be commented
        let e = road_len();
        assert_eq!(e,7);

        let r = _rail_len("Pointless Track");
        assert_eq!(r,15);
    }
}

fn main(){
    unimplemented!();
}

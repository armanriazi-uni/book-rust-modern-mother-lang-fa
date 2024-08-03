#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo test -q -p packtpub-ownership_bin --bin packtpub-ownership-main```
///
/// ```cargo doc  --package packtpub-ownership_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package packtpub-ownership_bin```
///
/// ## What
/// `'TODO`
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

#[derive(Debug,PartialEq)]
pub struct GBP(i32);
 fn on_money(a:i32,b:i32)->GBP {
    let mut g = GBP(a);
    let r;
    r = &g;
    let res = GBP(r.0 + b);
    res
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let g = on_money(3,4);
        assert_eq!(g, GBP(7));
    }
}
fn main(){
    unimplemented!();
}

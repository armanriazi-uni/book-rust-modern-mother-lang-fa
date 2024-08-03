#![allow(dead_code, unused_variables)]
use std::fmt;
#[warn(unused_imports)]
use std::fmt::{Display};

/// other-option-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p other-option_bin --bin other-option-ex-1```
///
/// ```cargo test -q -p other-option_bin --bin other-option-ex-1```
///
/// ```cargo doc  --package other-option_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package other-option_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your option/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  // ```rust,compile_fail,ignore
#[derive(Debug)]
#[allow(dead_code)]
enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

impl PartialEq<AB> for AB {
    fn eq(&self, other: &AB) -> bool{

        if  *self.ab == *other.ab {
         return true;
        }
        false
    }
    fn ne(&self, other: &AB) -> bool{
        if  *self.ab != *other.ab {
         return true;
        }
        false
    }
}
struct AB{
  ab: Box<[i64]>
}
fn category(a:&AB, b:&AB) -> Option<Comparison> {
        if a.eq(&b) {
            return Some(Comparison::Equal);
        }
        if a.ne(&b) {

            if  &a.ab.len() > &b.ab.len() {
            return Some(Comparison::Superlist);
            }
            if  &a.ab.len() < &b.ab.len()  {
            return Some(Comparison::Sublist);
            }
            if  &a.ab.len() != &b.ab.len()  {
            return Some(Comparison::Unequal);
            }
        }
    None
}
fn main() {
        let a=AB{ab:Box::<[i64;5]>::new([1,2,3,4,5])};
        let b=AB{ab:Box::<[i64;5]>::new([1,2,3,4,5])};
        println!("{:?}", category(&a,&b));
        //
        let a=AB{ab:Box::<[i64;5]>::new([1,2,3,4,6])};
        let b=AB{ab:Box::<[i64;5]>::new([1,2,3,4,5])};
        println!("{:?}", category(&a,&b));
        //
        let a=AB{ab:Box::<[i64;5]>::new([1,2,3,4,0])};
        let b=AB{ab:Box::<[i64;5]>::new([1,2,3,4,5])};
        println!("{:?}", category(&a,&b));
        //
        let a=AB{ab:Box::<[i64;3]>::new([1,2,3])};
        let b=AB{ab:Box::<[i64;5]>::new([1,2,3,4,5])};
        println!("{:?}", category(&a,&b));
        //
        let a=AB{ab:Box::<[i64;5]>::new([1,2,3,4,5])};
        let b=AB{ab:Box::<[i64;3]>::new([1,2,3])};
        println!("{:?}", category(&a,&b));
}
#[cfg(test)]
mod tests {
use super::*;
    #[test]
    fn comparasion(){
        let a=AB{ab:Box::<[i64;5]>::new([1,2,3,4,5])};
        let b=AB{ab:Box::<[i64;5]>::new([1,2,3,4,5])};
        assert!(true,"{}", a.eq(&b));
        let a2=AB{ab:Box::<[i64;5]>::new([1,2,3,4,0])};
        let b2=AB{ab:Box::<[i64;5]>::new([1,2,3,4,5])};
        assert!(true,"{}", a2.ne(&b2));
    }
}

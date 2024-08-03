#![allow(dead_code, unused_variables, unused_imports)]

use std::convert::TryFrom;
/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-doc-convert-try_from_bin --bin rust-doc-convert-try_from-bin-ex1```
///
/// ```cargo test -q -p rust-egg-convert-try_from_bin --bin rust-doc-convert-try_from-bin-ex1```
///
/// ```cargo doc  --package rust-egg-convert-try_from_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-convert-try_from_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` -
///
/// # Return
/// `Ten is less than one hundred.`
///
/// ## Example
/// `nothing`
/// 
#[derive(Debug)]
struct GreaterThanZeroOrEvenNumber(i8);
impl TryFrom<i8> for GreaterThanZeroOrEvenNumber {
    type Error = &'static str;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value <= 0 {
            Err("GreaterThanZeroOrEvenNumber only accepts values greater than zero!")
        } else {
            Ok(GreaterThanZeroOrEvenNumber(value))
        }
    }
}
impl GreaterThanZeroOrEvenNumber{   
      fn try_from_2(value: i8) -> Result<Self,  &'static str> {
        if value % 2 == 0 {
            println!("even number!");
            Ok(GreaterThanZeroOrEvenNumber(value))
        } else {
            println!("Not an even number: {}", value);
            Err("GreaterThanZeroOrEvenNumber only accepts values mod 2!")
        }
    }
}
fn main(){
    let a= GreaterThanZeroOrEvenNumber::try_from(-5).ok();
    let b= GreaterThanZeroOrEvenNumber::try_from(-55).is_err();    
    let c= GreaterThanZeroOrEvenNumber::try_from_2(56).ok().unwrap();    
    println!("{:?}",a);
    println!("{:?}",b);    
    println!("{:?}",c);    
}
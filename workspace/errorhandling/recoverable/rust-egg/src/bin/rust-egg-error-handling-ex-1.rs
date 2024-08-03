#![allow(dead_code, unused_variables)]


/// rust-egg-error-handling-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-error-handling_bin --bin  rust-egg-error-handling-ex-1```
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
/// `unimplemented`
///
/// ## Example
/// //``rust,no_run,compile_fail,ignore


use std::error;
use std::fmt;
use std::io;//::{self,BufRead};
#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str((self as &dyn error::Error).description())
    }
}
impl error::Error for CreationError {
    fn description(&self) -> &str {
        match *self {
            CreationError::Negative => "Negative",
            CreationError::Zero => "Zero",
        }
    }
}
// PositiveNonzeroInteger is a struct defined below the tests.
fn read_and_validate(
    b: &mut dyn io::BufRead,
) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
    let mut line = String::new();
    b.read_line(&mut line)?;//decode. to string
    let num: i64 = line.trim().parse()?;
    let answer = PositiveNonzeroInteger::new(num)?;
    Ok(answer)
}
// This is a test helper function that turns a &str into a BufReader.
fn test_with_str(s: &str) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
    let mut b = io::BufReader::new(s.as_bytes());//encode
    read_and_validate(&mut b)
}
fn test_success() {
    let x = test_with_str("42\n");
    //Ok(PositiveNonzeroInteger(42))
    assert_eq!(PositiveNonzeroInteger(42), x.unwrap());
}
fn test_not_num() {
    let x = test_with_str("eleven billion\n");
    //:Err(ParseIntError { kind: InvalidDigit })
    //println!("Printed:{:?}",x);
    assert!(x.is_err());
}
fn test_non_positive() {
    let x = test_with_str("-40\n");
    //:Err(ParseIntError { kind: InvalidDigit })
    assert!(x.is_err());
}
// fn read_numbers_from_file(
//     file: &mut dyn io::BufRead,
// ) -> Result<Vec<i64>, io::Error> {
//     let mut numbers = vec![];
//     for line_result in file.lines() {
//         let line = line_result?;
//         numbers.push(line.parse()?);
//     }
//     Ok(numbers)
// }

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);
impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value == 0 {
            Err(CreationError::Zero)
        } else if value < 0 {
            Err(CreationError::Negative)
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}
fn test_positive_nonzero_integer_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
fn main() {
   //let item=func()?;
   test_success();
   test_not_num();
   test_not_num();
   test_positive_nonzero_integer_creation();
   test_non_positive();
}

#![allow(dead_code, unused_variables)]
use std::error;
use std::fmt;
use std::io;//::{self,BufRead};

/// rust-egg-error-handling-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-error-handling_bin --bin  rust-egg-error-handling-ex-3```
///
/// ```cargo test -q -p rust-egg-error-handling_bin --bin  rust-egg-error-handling-ex-3```
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

#[derive(PartialEq, Debug)] // TO_REPORT_BUG:`CreationError` cannot be formatted using `{:?}` *BecauseOf(-):Debug   *BecauseOf(?):struct::method'

struct PositiveNonzeroInteger(u64);

#[derive(PartialEq,Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value < 0 {
            Err(CreationError::Negative)
        } else if value == 0 {
            Err(CreationError::Zero)
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}
fn main(){
unimplemented!()
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}

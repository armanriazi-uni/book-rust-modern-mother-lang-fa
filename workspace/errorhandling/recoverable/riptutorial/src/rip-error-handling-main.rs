#![allow(dead_code, unused_variables)]
use std::error::Error;
use std::fmt;
/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rip-error-handling_bin --bin rip-error-handling-main```
///
/// ```cargo doc  --package rip-error-handling_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rip-error-handling_bin ```
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
/// sum two numbers: Ok(
///    579,
///)
///
/// ## Example
///  `TODO`
///


#[derive(Debug)]
enum DateError {
    InvalidDay(u8),
    InvalidMonth(u8),
}
impl fmt::Display for DateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &DateError::InvalidDay(day) => write!(f, "Day {} is outside range!", day),
            &DateError::InvalidMonth(month) => write!(f, "Month {} is outside range!", month),
        }
    }
}
impl Error for DateError {
    fn description(&self) -> &str {
        match self {
            &DateError::InvalidDay(_) => "Day is outside range!",
            &DateError::InvalidMonth(_) => "Month is outside range!",
        }
    }
}
struct Date {
    day: u8,
    month: u8,
    year: i16,
}
fn validate(date: &Date) -> Result<(), DateError> {
    if date.month < 1 || date.month > 12 {
        Err(DateError::InvalidMonth(date.month))
    } else if date.day < 1 || date.day > 31 {
        Err(DateError::InvalidDay(date.day))
    } else {
        Ok(())
    }
}
fn add_days(date: Date, days: i32) -> Result<Date, DateError> {
    validate(&date)?; // notice `?` -- returns early on error
    // the date logic ...
    Ok(date)
}
fn main(){
    let date=Date{day:4,month:1,year:2023};
    let _= add_days(date,365);
}

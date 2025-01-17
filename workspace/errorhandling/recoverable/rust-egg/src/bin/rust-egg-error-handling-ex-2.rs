#![allow(dead_code, unused_variables)]
use std::error;
use std::fmt;
use std::io;//::{self,BufRead};

/// rust-egg-error-handling-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-error-handling_bin --bin  rust-egg-error-handling-ex-2```
///
/// ```cargo test -q -p rust-egg-error-handling_bin --bin  rust-egg-error-handling-ex-2```
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

// PositiveNonzeroInteger is a struct defined below the tests.
fn read_and_validate(
    b: &mut dyn io::BufRead,
) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
    let mut line = String::new();
    b.read_line(&mut line)?;
    let num: i64 = line.trim().parse()?;
    let answer = PositiveNonzeroInteger::new(num)?;
    Ok(answer)
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

// This is a test helper function that turns a &str into a BufReader.
fn test_with_str(s: &str) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
    let mut b = io::BufReader::new(s.as_bytes());
    read_and_validate(&mut b)
}

#[test]
fn test_success() {
    let x = test_with_str("42\n");
    assert_eq!(PositiveNonzeroInteger(42), x.unwrap());
}

#[test]
fn test_not_num() {
    let x = test_with_str("eleven billion\n");
    assert!(x.is_err());
}

#[test]
fn test_non_positive() {
    let x = test_with_str("-40\n");
    assert!(x.is_err());
}

#[test]
fn test_ioerror() {
    struct Broken;
    impl io::Read for Broken {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "uh-oh!"))
        }
    }
    let mut b = io::BufReader::new(Broken);
    assert!(read_and_validate(&mut b).is_err());
    //println!("{}",read_and_validate(&mut b).unwrap_err().to_string());

    assert_eq!("uh-oh!", read_and_validate(&mut b).unwrap_err().to_string());
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value == 0 {
            Err(CreationError::Zero)
        } else if value < 0 {
            Err(CreationError::Negative(value))
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
}

#[test]
fn test_positive_nonzero_integer_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative(-10)),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative(i64),
    Zero,
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &CreationError::Negative(negative) => write!(f, "Number {} is Negative!", negative),
            &CreationError::Zero => write!(f, "Number is zero!"),
        }
        //f.write_str((self as &dyn error::Error).description())
    }
}

impl error::Error for CreationError {
    fn description(&self) -> &str {
        match self {
            &CreationError::Negative(_) => "Negative",
            &CreationError::Zero => "Zero",
        }
        /*match *self {
            CreationError::Negative(_) => "Negative",
            CreationError::Zero => "Zero",
        }*/
    }
}

fn main(){
    unimplemented!()
    }
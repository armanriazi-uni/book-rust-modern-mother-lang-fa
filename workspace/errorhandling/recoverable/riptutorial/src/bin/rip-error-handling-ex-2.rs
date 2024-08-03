#![allow(dead_code, unused_variables)]

/// rip-error-handling-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rip-error-handling_bin --bin  rip-error-handling-ex-2```
///
/// ## What
/// `Error Handling`
///
/// ## How
/// We can unwrap the result value, which will return the contained value. However, it’ll panic if it’s an error. We recommend not using unwrap unless you need the code to panic (like in testing).
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `unimplemented`
///
/// ## Example
/// //```rust,no_run,compile_fail,ignore
use std::error::Error;
use std::fmt;
use std::convert::From;
use std::io::Error as IoError;
use std::str::Utf8Error;
use std::str;
 /// Allow the use of "{:?}" format specifier
#[derive(Debug)]
enum CustomError {
    Io(IoError),
    Utf8(Utf8Error),
    Other,
}

/// Allow the use of "{}" format specifier
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::Io(ref cause) => write!(f, "I/O Error: {}", cause),
            CustomError::Utf8(ref cause) => write!(f, "UTF-8 Error: {}", cause),
            CustomError::Other => write!(f, "Unknown error!"),
        }
    }
}

/// Allow this type to be treated like an error
impl Error for CustomError {
    fn description(&self) -> &str {
        match *self {
            CustomError::Io(ref cause) => cause.description(),
            CustomError::Utf8(ref cause) => cause.description(),
            CustomError::Other => "Unknown error!",
        }
    }
/// Use an Option<&Error>. This is the return type of Error.cause().
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            CustomError::Io(ref cause) => Some(cause),
            CustomError::Utf8(ref cause) => Some(cause),
            CustomError::Other => None,
        }
    }
}

/// Support converting system errors into our custom error.
///From trait in the standard library, which is used to convert errors from one type into another
/// This trait is used in `try!`.
impl From<IoError> for CustomError {
    fn from(cause: IoError) -> CustomError {
        CustomError::Io(cause)
    }
}
impl From<Utf8Error> for CustomError {
    fn from(cause: Utf8Error) -> CustomError {
        CustomError::Utf8(cause)
    }
}
fn main(){
   // Simulate an I/O error
   let io_error = IoError::new(std::io::ErrorKind::NotFound, "File not found");
   let custom_io_error = CustomError::from(io_error);
   println!("Handling I/O error: {:?}", custom_io_error);

//    // Simulate a UTF-8 decoding error
//    let bytes = b"\xFF"; // Example byte sequence
//    let utf8_error = Utf8Error::invalid_utf8_sequence(bytes, std::str::Utf8Encoding::UTF_8);
//    let custom_utf8_error = CustomError::from(utf8_error);
//    println!("Handling UTF-8 error: {:?}", custom_utf8_error);

   // Handling an unknown error
   let custom_other_error = CustomError::Other;
   println!("Handling other error: {:?}", custom_other_error);
}

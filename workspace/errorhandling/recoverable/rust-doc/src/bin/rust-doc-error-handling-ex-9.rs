#![allow(dead_code, unused_variables)]

/// rust-doc-error-handling-ex-
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-error-handling_bin --bin  rust-doc-error-handling-ex-9```
///
/// ## What
/// `Propagating errors`
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
use std::error::Error;
use std::fmt;
use std::io;
use std::num;

// pub type XResult<T> = Result<T, XError>;

// pub fn xerror<T>(message: &str) -> XResult<T> {
//     Err(XError::new(&message))
// }

// #[macro_export]
// macro_rules! xerr {
//     ($x:expr) => {{return xerror($x).expect("REASON");}};
//     ($x:expr, $($y:expr),+) => {{return xerror(&format!($x, $($y),+));}};
// }

// #[derive(Debug)]
// pub enum XError {
//   //  Image(image::ImageError),
//     Io(::std::io::Error),
//     //Json(json::Error),
//    // Log(log::SetLoggerError),
//     ParseFloat(::std::num::ParseFloatError),
//     ParseInt(::std::num::ParseIntError),
//    // Rayon(rayon::ThreadPoolBuildError),
//     Retest(String),
// }

// impl Error for XError {}

// impl XError {
//     pub fn new(message: &str) -> XError {
//         XError::Retest(message.to_string())
//     }
// }

// impl fmt::Display for XError {
//     fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             XError::Image(ref err) => write!(out, "Image error: {}", err),
//             XError::Io(ref err) => write!(out, "File error: {}", err),
//           //  XError::Json(ref err) => write!(out, "JSON error: {}", err),
//             // XError::Log(ref err) => {
//             //     write!(out, "Failed to set logger: {}", err)
//             // }
//             XError::ParseFloat(ref err) => {
//                 write!(out, "Failed to read decimal number: {}", err)
//             }
//             XError::ParseInt(ref err) => {
//                 write!(out, "Failed to read whole number: {}", err)
//             }
//             // XError::Rayon(ref err) => {
//             //     write!(out, "Failed to create thread pool: {}", err)
//             // }
//             XError::Retest(ref err) => write!(out, "{}", err),
//         }
//     }
// }

// // impl From<image::ImageError> for XError {
// //     fn from(err: image::ImageError) -> XError {
// //         XError::Image(err)
// //     }
// // }

// impl From<io::Error> for XError {
//     fn from(err: io::Error) -> XError {
//         XError::Io(err)
//     }
// }

// // impl From<json::Error> for XError {
// //     fn from(err: json::Error) -> XError {
// //         XError::Json(err)
// //     }
// // }

// // impl From<log::SetLoggerError> for XError {
// //     fn from(err: log::SetLoggerError) -> XError {
// //         XError::Log(err)
// //     }
// // }

// impl From<num::ParseFloatError> for XError {
//     fn from(err: num::ParseFloatError) -> XError {
//         XError::ParseFloat(err)
//     }
// }

// impl From<num::ParseIntError> for XError {
//     fn from(err: num::ParseIntError) -> XError {
//         XError::ParseInt(err)
//     }
// }

// // impl From<rayon::ThreadPoolBuildError> for XError {
// //     fn from(err: rayon::ThreadPoolBuildError) -> XError {
// //         XError::Rayon(err)
// //     }
// // }
fn main() {
    unimplement!();
    // let input = "123abc"; // A string that should be parsable to an integer

    // // Attempt to parse the integer using the `parse()` method,
    // // which returns a `Result`.
    // let result = input.parse::<i32>();

    // // Check the result of the parsing attempt.
    // match result {
    //     Ok(value) => println!("Successfully parsed value: {}", value),
    //     Err(e) => {
    //         // Use the custom error handling mechanism to handle the error.
    //         // The `xerr!` macro simplifies this process by directly returning an error.
    //         xerr!(e);
    //     }
    // }
}

#![allow(dead_code, unused_variables)]

/// rust-doc-error-handling-ex-10
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-error-handling_bin --bin  rust-doc-error-handling-ex-10```
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
use std::{fmt, io::Write}; // Added 'io::Write' for completeness

// Define a custom error type
#[derive(Debug, Clone)]
struct MyError {
    message: String,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MyError {}

fn main() {
    let err = Box::new(MyError {
        message: "An error occurred".to_string(),
    });

    // Demonstrate returning an error from a function
    match handle_error(err) {
        Ok(_) => println!("Operation completed successfully"),
        Err(e) => println!("Error encountered: {}", e),
    }
}

// Function that demonstrates returning an error
fn handle_error(error: Box<dyn std::error::Error + Send + Sync>) -> Result<(), Box<dyn std::error::Error>> {
    // Simulate an operation that could fail
    if true {
        // Return the error directly instead of trying to clone it
        return Err(error);
    }

    Ok(())
}

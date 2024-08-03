/// The function calculate_cube_of_integer calculates cube of the number provided in the argument
/// and return that to the calling function
///
/// # Arguments
///
/// * `operand1` - This is the integer number to calculate the cube
///
/// # Return
///
/// This function returns the cube of the integer number
pub fn calculate_cube_of_integer(operand1: u32) -> u32 {
    let result: u32 = operand1 * operand1 * operand1;
    result
}

/// Only runs on the 2018 edition.
///
/// ```edition2018,compile_fail
/// let result: Result<i32, ParseIntError> = try {
///     "1".parse::<i32>()?
///         + "2".parse::<i32>()?
///         + "3".parse::<i32>()?
/// };
/// ```
///

/// ```compile_fail
/// let x = my_crate::MyStruct(-5);
/// ```
#[cfg(doctest)]
pub struct MyStructOnlyTakesUsize;

/// Example 1:
///
/// ```
/// let x = 12;
/// ```
///
/// Example 2:
///
/// ```rust
/// let x = 12;
/// ```
pub fn item() {}

/// Example
///
///! It's possible that you want a code example to be compiled but not run (it's very common for I/O examples). You can tell rustdoc by adding no_run to your block
/// ```no_run
/// use std::fs::File;
///
/// let mut f = File::open("some_file.txt").expect("failed");
/// ```
pub fn norun() {}

/// Example
///
///! Last small thing: you can hide lines from the code example display.All lines starting with # won't be printed, but they'll be tested. For example:
/// ```no_run
/// use std::fs::File;
///
/// # fn foo() -> std::io::Result<()> {
/// let mut f = File::create("foo.txt")?;
/// # Ok(())
/// # }
/// ```
pub fn hideline() {}

// #![warn(rustdoc::invalid_html_tags)]
/// <h1>
/// </script>
pub fn foo() {}

/// Not show since private functions
fn not_included() {}

// Example from the futures-rs library
///// #[doc(hidden)]
//`pub use self::async_await::*;`

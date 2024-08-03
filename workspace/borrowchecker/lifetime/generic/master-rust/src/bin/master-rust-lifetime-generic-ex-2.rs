#![allow(dead_code, unused_variables)]

/// master-rust-lifetime-generic-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p  master-rust-lifetime-generic_bin --bin  master-rust-lifetime-generic-ex-2```
///
/// ## What
/// `lifetime_bounds`
///
/// ## How
/// `This is read as the lifetime 'a outlives 'b or in other words 'b should never live longer than 'a.`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `unimplemented`
///
/// ## Example
///
///```rust,no_run,compile_fail,ignore
/// fn main() {
///     let other = String::from("Local");
///     let log2 = Logger(&other, Level::Error);// borrowed value does not live long enough
///     configure_logger(&log2);
/// }
/// ```
/// > The error message clearly say, that the borrowed value must be valid for the static lifetime, but we have passed it a string, which has a lifetime called 'a from main, which is a shorter lifetime than 'static.


enum Level {
    Error
}

struct Logger<'a>(&'a str, Level);

fn configure_logger<T>(_t: T) where T: Send + 'static {
    // configure the logger here
}

fn main() {
    let name = "Global";
    let log1 = Logger(name, Level::Error);
    configure_logger(log1);
}

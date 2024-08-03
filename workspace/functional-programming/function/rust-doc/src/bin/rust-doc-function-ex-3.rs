#![allow(dead_code, unused_variables)]
/// rust-doc-function-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-function_bin --bin rust-doc-function-ex-3```
///
/// ```cargo doc  --package rust-doc-function_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-function_bin ```
///
/// ## What
/// `Initializer-function`
///
/// ## How
/// `The following code demonstrates how we can do this entirely at runtime.using the include_bytes! macro, which also reads the file at compile time.Without the const function, all this would be done at runtime.`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///
///
fn main() {
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

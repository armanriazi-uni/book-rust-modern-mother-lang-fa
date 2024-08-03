#![allow(dead_code, unused_variables)]



/// rust-doc-std-command-process-wait-ex-8
///
/// ## Commands
///
/// ```cargo run -q -p rust_doc_std_bin  --bin rust-doc-std-command-process-wait-ex-8```
///
/// ```cargo doc  --package rust_doc_std_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust_doc_std_bin ```
///
/// ## What
/// $ rustc wait.rs && ./wait
/// # `wait` keeps running for 5 seconds until the `sleep 5` command finishes
/// reached end of main
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// ```

use std::process::Command;
fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();
    println!("reached end of main");
}
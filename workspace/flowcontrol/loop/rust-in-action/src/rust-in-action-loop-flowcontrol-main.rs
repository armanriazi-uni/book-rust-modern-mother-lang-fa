#![allow(dead_code, unused_variables)]
use std::time::{Duration, Instant};

/// Main
///
/// # Commands
///
/// ```cargo run -q -p rust-in-action-loop-flowcontrol_bin --bin rust-in-action-loop-flowcontrol-main```
///
/// ```cargo doc  --package rust-in-action-loop-flowcontrol_bin --message-loopmat short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-loop-flowcontrol_bin```
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
/// `20`
///
/// ## Example
/// // ```rust,compile_fail,ignore
/// loop {
///  let requester, request = accept_request();
///  let result = process_request(request);
///  send_response(requester, result);
///}
///```

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

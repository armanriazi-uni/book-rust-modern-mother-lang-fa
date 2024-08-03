#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo test -q -p test_binary  --test test_binary-main -- --test-threads=1  --show-output --nocapture```
///
/// ```cargo doc  --package test_binary --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc --package test_binary```
///
/// ## What
/// `Test Binary`
///
/// ## How
/// `Binary crates cannot be tested in the same way as library crates, because the items in binary crates cannot to be imported by other crates. Instead, we can test the binary crate by running it with the assert_cmd crate. This crate allows us to run the binary crate as a subprocess and assert that it exits successfully and prints the expected output.`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// `nothing`
///

use assert_cmd::Command;

#[test]
fn output_is_crab_as_utf8_bytes() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("test_binary-main")?; // Initialize command with the name of the tested binary crate

    let expected: String = "🦀"
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:08b}"))
        .fold("".to_string(), |mut acc, byte| {
            acc.push_str(&byte);
            acc.push(' ');
            acc
        })
        .trim_end()
        .to_string();

    // Execute the command and assert that it succeeds and prints the expected output
    cmd.assert().success().stdout(expected);

    Ok(())
}

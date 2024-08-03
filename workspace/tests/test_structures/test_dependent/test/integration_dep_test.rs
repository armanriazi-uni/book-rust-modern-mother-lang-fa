/// dependent test
///
/// ## Commands
///
///
/// ```cd test_dependent```
/// 
/// ```cargo test  -p test_dependent -- --show-output --nocapture```
///
/// ```cargo test  -p test_dependent -- --show-output --ignore```
///             
/// ```cargo doc  --package test_dependent --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package test_dependent```
///
/// ## What
/// `TODO`
///
/// ## How
/// `Test functions have to return a type that implements the std::process::Termination trait (the main function has this same restriction).`
///
/// # Arguments
///
/// * `TODO`
///
/// # Return
/// 
/// `passed`
/// 


use std::io;

fn read_last_line(filename: &str) -> Result<String, io::Error> {
    let file_contents = std::fs::read_to_string(filename)?;
    file_contents
        .lines()
        .last()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No lines found in input"))
        .map(|s| s.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::{error::Error as StdError, fs, io::Write};

    #[test]
    fn test_read_last_line() -> Result<(), Box<dyn StdError>> {
        // Box<dyn StdError> allows propagating any error that implements StdError
        let test_file_name = "/tmp/test.txt";

        // Open tmp file for testing
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(test_file_name)?;

        // Initialize test result
        let mut test_result = Ok(());

        let lines = ["🦖", "🦕", "🦎"];
        // Write lines to file one at a time and check that the last line is returned correctly
        for line in &lines {
            file.write_all(line.as_bytes())?;
            file.write_all(b"\n")?;
            let last_line = read_last_line(test_file_name).expect("3");
            if &last_line.as_str() != line {
                test_result = Err("last line is not '🦖!".into()); // into is required to convert &str to Box<dyn StdError>
                break;
            };
        }

        // Remove tmp file
        fs::remove_file(test_file_name)?;

        // Return test result
        test_result
    }
}
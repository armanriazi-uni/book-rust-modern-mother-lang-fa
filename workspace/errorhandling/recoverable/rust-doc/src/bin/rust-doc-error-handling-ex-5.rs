#![allow(dead_code, unused_variables)]



/// rust-doc-error-handling-ex-5
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-error-handling_bin --bin  rust-doc-error-handling-ex-5```
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
/// ## Example
/// //``rust,no_run,compile_fail,ignore


//use error_chain::error_chain;
// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         Reqwest(reqwest::Error);
//         ParseIntError(std::num::ParseIntError);
//     }
//     errors { RandomResponseError(t: String) }
// }

// fn parse_response(response: reqwest::blocking::Response) -> Result<u32> {
//   let mut body = response.text()?;
//   body.pop();
//   body
//     .parse::<u32>()
//     .chain_err(|| ErrorKind::RandomResponseError(body))
// }

use std::io;
use reqwest::{Client, Error as ReqwestError};
use std::convert::Infallible;
// Implementing parse_response to parse the response body as a u32
async fn parse_response(response: reqwest::Response) -> Result<u32, Box<dyn std::error::Error>> {
    let body = response.text().await?;
    // Assuming the response body contains a single integer value
    let random_value: u32 = body.trim().parse()?;
    Ok(random_value)
}
#[derive(Debug)]
enum RandomResponseError {
    CustomError,
}
impl std::fmt::Display for RandomResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Custom error")
    }
}
impl std::error::Error for RandomResponseError {}
fn main() {
    // Assuming async runtime setup is done elsewhere
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        if let Err(error) = run().await {
            match error.downcast::<ReqwestError>() {
                Ok(ReqwestError::StatusCode(status_code)) => println!("HTTP Status Code Error: {:?}", status_code),
                _ => println!("Other error: {:?}", error),
            }
        }
    });
}
async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://www.random.org/integers/?num=1&min=0&max=10&col=1&base=10&format=plain");
    let response = client.get(&url).send().await?;
    let random_value: u32 = parse_response(response).await?;
    println!("A random number between 0 and 10: {}", random_value);
    Ok(())
}

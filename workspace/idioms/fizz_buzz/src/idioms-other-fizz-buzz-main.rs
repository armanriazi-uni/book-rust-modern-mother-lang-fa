#![allow(dead_code, unused_variables)]
use std::io;
use std::io::BufRead;
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p idioms-other-fizz-buzz_bin --bin idioms-other-fizz-buzz-main```
///
/// ## What
/// `loop int`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `n=15` - This is the i32 to Fizz or Buzz or FizzBuzz the fizz_buzz
///
/// # Return
/// `["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz", "Buzz"]`
///
/// ## Example
///```rust

struct Solution;
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .into_iter()
            .map(|i| match (i % 3 == 0, i % 5 == 0) {
                (true, true) => "FizzBuzz".to_string(),
                (true, false) => "Fizz".to_string(),
                (false, true) => "Buzz".to_string(),
                _ => i.to_string(),
            })
            .collect()
    }
}
fn main(){
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let result= Solution::fizz_buzz(n);
    println!("{:?}",result);
}

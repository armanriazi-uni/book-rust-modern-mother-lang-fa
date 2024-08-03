#![allow(dead_code, unused_variables)]

/// idioms-other-fizz-buzz-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p idioms-other-fizz-buzz_bin --bin idioms-other-fizz-buzz-ex-1```
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
			.map(|n| match (n % 3, n % 5) {
				(0, 0) => "FizzBuzz".to_owned(),
				(0, _) => "Fizz".to_owned(),
				(_, 0) => "Buzz".to_owned(),
				_ => n.to_string(),
			})
			.collect()
	}
}


fn main(){
    let result= Solution::fizz_buzz(100);
    println!("{:?}",result);
    //unimplemented!();
}

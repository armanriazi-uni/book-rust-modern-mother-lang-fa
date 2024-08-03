#![allow(dead_code, unused_variables)]

/// Bin
///
/// ## Commands
///
/// ```cargo test -q -p unit_test --bin test_panic -- --show-output --ignored```
///
/// ```cargo doc  --package unit_test --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package unit_test```
///
/// ## What
/// `Unit Test`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// `TODO`

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(102);
    }

	#[test]
	#[ignore]
	fn expensive_test() {
	    assert_eq!(1,1);
	}

}


fn main(){
    unimplemented!();
}
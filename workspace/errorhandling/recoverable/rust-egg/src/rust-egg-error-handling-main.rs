#![allow(dead_code, unused_variables)]
use std::num::ParseIntError;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-error-handling_bin --bin rust-egg-error-handling-main```
///
/// ```cargo test -q -p rust-egg-error-handling_bin --bin rust-egg-error-handling-main```
///
/// ```cargo doc  --package rust-egg-error-handling_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-error-handling_bin ```
///
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
/// `nothing`
///
/// ## Example
///  `TODO`
///
///
/// Say we're writing a game where you can buy items with tokens. All items cost
/// 5 tokens, and whenever you purchase items there is a processing fee of 1
/// token. A player of the game will type in how many items they want to buy,
/// and the `total_cost` function will calculate the total number of tokens.
/// Since the player typed in the quantity, though, we get it as a string-- and
/// they might have typed anything, not just numbers!

/// Right now, this function isn't handling the error case at all (and isn't
/// handling the success case properly either). What we want to do is:
/// if we call the `parse` function on a string that is not a number, that
/// function will return a `ParseIntError`, and in that case, we want to
/// immediately return that error from our function and not try to multiply
/// and add.

/// There are at least two ways to implement this that are both correct-- but
/// one is a lot shorter! Execute `rustlings hint errors2` for hints to both ways.

fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(())
}
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;
    Ok(qty * cost_per_item + processing_fee)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }
    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}


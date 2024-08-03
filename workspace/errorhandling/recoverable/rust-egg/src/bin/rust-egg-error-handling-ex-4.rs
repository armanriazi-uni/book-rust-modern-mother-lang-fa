#![allow(dead_code, unused_variables)]


/// rust-egg-error-handling-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-error-handling_bin --bin  rust-egg-error-handling-ex-4```
///
/// ```cargo test -q -p rust-egg-error-handling_bin --bin  rust-egg-error-handling-ex-4```
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
/// `unimplemented`
///
/// ## Example
/// //``rust,no_run,compile_fail,ignore

// This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was, instead of just sometimes returning `None`. The 2nd test currently
// does not compile or pass, but it illustrates the behavior we would like
// this function to have.
// Execute `rustlings hint errors1` for hints!
use std::error;
use std::fmt;
use std::io;//::{self,BufRead};
pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.len() > 0 {
        Ok(format!("Hi! My name is {}", name))
    } else {
        // Empty names aren't allowed.
        Err(String::from("`name` was empty; it must be nonempty."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }
    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
fn main(){
unimplemented!()
}

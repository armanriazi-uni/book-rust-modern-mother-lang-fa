#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo test -q -p rust-egg-return_bin --bin rust-egg-return-main```
///
/// ```cargo doc  --package rust-egg-return_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-return_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your return/func name]
///
/// # Return
/// `unit test`
///
/// ## Example
/// //```rust,compile_fail,ignore
pub fn generate_nametag_text(name: String) -> Result<String,String> {

    if name.len() > 0 {
            Ok(format!("Hi! My name is {}", name))
            }
            else {
                Err(String::from("'name' was empty; it must be noneempty"))
            }
}

pub fn opt_generate_nametag_text(name: String) -> Option<String> {

    if name.len() > 0 {
            Some(format!("Hi! My name is {}", name))
            }
            else {
                None
            }
}

fn main(){
 unimplemented!();
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn generate_name_tag_text_for_nonempty_name(){
    assert_eq!(
        generate_nametag_text("arman".into()),
        Ok("Hi! My name is arman".into())
        );
    }
#[test]
    fn generate_name_tag_text_for_nonempty_fails(){
        assert_eq!(
            generate_nametag_text("".into()),
            Err("'name' was empty; it must be noneempty".into())
            );
    }


#[test]
fn opt_generate_name_tag_text_for_nonempty_name(){
    assert_eq!(
        opt_generate_nametag_text("arman".into()),
        Some("Hi! My name is arman".into())
        );
    }
#[test]
    fn opt_generate_name_tag_text_for_nonempty_fails(){
        assert_eq!(
            opt_generate_nametag_text("".into()),
            None
            );
    }
}

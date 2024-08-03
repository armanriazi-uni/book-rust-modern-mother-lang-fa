#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p code-like-a-pro-types-string_bin --bin code-like-a-pro-types-string-main```
///
/// ```cargo doc  --package code-like-a-pro-types-string_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package code-like-a-pro-types-string_bin```
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
/// `
/// lowercased('HeLlO') -> 'hello'
/// lowercased_ascii('CoMpUtErS`) -> 'computers'
/// `
/// ## Example
/// ```rust,compile_fail,ignore
///
fn lowercased(s: String) -> String {
    s.to_lowercase()
}

fn lowercased_ascii(mut s: String) -> String {
    s.make_ascii_lowercase();
    s
}

fn main() {
    let s1 = "HeLlO";
    let s2 = "CoMpUtErS";
    println!("lowercased('{}') -> '{}'", s1, lowercased(s1.to_owned()));
    println!(
        "lowercased_ascii('{}`) -> '{}'",
        s2,
        lowercased_ascii(s2.to_owned())
    );
}

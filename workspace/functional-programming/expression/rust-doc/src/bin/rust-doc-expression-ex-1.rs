#![allow(dead_code, unused_variables)]
/// rust-doc-expression-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-expression_bin --bin rust-doc-expression-ex-1```
///
/// ```cargo doc  --package rust-doc-expression_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-expression_bin ```
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

fn main() {
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);
    //
    let  number=3;
    println!("Number {}", number);
    {
        let number = 5;
        println!("Number {}",number);
    }
    println!("Number {}",number);
}

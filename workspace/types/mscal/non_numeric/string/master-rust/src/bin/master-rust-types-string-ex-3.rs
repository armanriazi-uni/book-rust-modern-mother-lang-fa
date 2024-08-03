#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]


/// ex-3
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-types-string_bin --bin master-rust-types-string-ex-3```
///
/// ```cargo egg  --package master-rust-types-string_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --egg  --package master-rust-types-string_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `Internally, &String automatically coerces to &str, due to the type coercion trait Deref implemented for &String to &str. This is because String implements Deref for the str type.`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// 'nothing'
///
/// ## Example
/// //```rust,compile_fail,ignore
fn main(){
let foo = "Foo";
    let bar = "Bar";
    let baz = foo.to_string() + bar;//foo + bar => means error
    println!("{}",baz);
}

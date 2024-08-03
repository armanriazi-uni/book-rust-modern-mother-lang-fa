#![allow(dead_code, unused_variables)]

/// master-rust-struct-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-struct_bin --bin master-rust-struct-ex-1```
///
/// ```cargo doc  --package master-rust-struct_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-struct_bin```
///
/// ## What
// `pattern_matching`
///
/// ## How
/// `the compiler thinks that we want to match against Person(&String) but the a value is actually Person(String). So, in this case ref has to be used to destructure it as a reference. To make it compile, we change it accordingly to Person(ref name) on the left-hand side.`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `assert:true`
///
/// ## Example
///```rust
///fn main() {
///    let a = Person("Richard Feynman".to_string());
///    match &a {
///        Person(name) => println!("{} was a great physicist !", name),
///         _ => panic!("Oh no !")
///    }
///    let b = a;
///}

struct Person(String);

fn main() {
    let a = Person("Richard Feynman".to_string());
    match a {
        Person(ref name) => println!("{} was a great physicist !", name),
         _ => panic!("Oh no !")
    }

    let b = a;
}

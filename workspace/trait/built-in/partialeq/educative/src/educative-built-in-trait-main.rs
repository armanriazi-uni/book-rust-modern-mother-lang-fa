#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p educative-built-in-trait_bin --bin educative-built-in-trait-main```
///
/// ```cargo doc  --package educative-built-in-trait_lib --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-built-in-trait_lib```
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Person {
    name: String,
    age: u32,
}
fn main() {
    let alice = Person {
        name: "Alice".to_owned(),
        age: 30,
    };
    let also_alice = alice.clone();
    assert_eq!(alice, also_alice);
    assert!(alice >= also_alice);
    assert!(alice <= also_alice);
    let bob = Person {
        name: "Bob".to_owned(),
        age: 25,
    };
    assert_ne!(alice, bob);
    // What do you think is bigger, Alice or Bob?
    println!("{:?} > {:?} == {}", alice, bob, alice > bob);
}

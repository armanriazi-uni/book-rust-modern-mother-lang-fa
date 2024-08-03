#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-types-generic_bin --bin educative-types-generic-main```
///
/// ```cargo doc  --package educative-types-generic_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-types-generic_bin```
///
/// ## What
// `TODO`
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
/// ```rust
/// fn greet<Age>(person: &Person<String, Age>) {
///     println!("Hello, {}", person.name);
/// }
/// ```
/// Next:
/// ```rust,compile_fail,no_run
/// fn greet(person: &Person<String, Age>) {
///     println!("Hello, {}", person.name);
/// }
/// // Result: error[E0412]: cannot find type `Age` in this scope
/// ```

struct Person<Name, Age> {
    name: Name,
    age: Age,
}
fn main() {
    let alice: Person<String, u32> = Person {
        name: "Alice".to_owned(),
        age: 30_u32,
    };
    greet(&alice);
    let bob = Person {
        name: "Bob".to_owned(),
        age: 35_u64,
    };
    greet(&bob);
}
fn greet<Age: std::fmt::Display>(person: &Person<String, Age>) {
    println!("Hello, {}, you are {} years old", person.name, person.age);
}

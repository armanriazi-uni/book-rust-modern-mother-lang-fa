#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-enum_bin --bin educative-enum-ex-1```
///
/// ```cargo doc --package educative-enum_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc --package educative-enum_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your enum/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// //```rust,compile_fail,ignore
enum Job {
    Doctor,
    Scientist,
    Teacher,
}

struct Person {
    name: String,
    job: Job,
}

fn main() {
    let people = [
        Person { name: "Alice".to_owned(), job: Job::Doctor },
        Person { name: "Bob".to_owned(), job: Job::Scientist },
        Person { name: "Charlie".to_owned(), job: Job::Doctor },
    ];

    for person in &people {
        match person.job {
            Job::Doctor => {
                println!("Hello, {}", person.name);
            }
            Job::Scientist => {
                // don't do anything
            }
            Job::Teacher => {
            println!("Hello, {}", person.name);
            }
        }
    }
}

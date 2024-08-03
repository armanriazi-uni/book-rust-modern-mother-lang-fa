#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-enum_bin --bin educative-enum-main```
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
fn two_salaries(x: &Job, y: &Job) -> Option<u32> {
    let salary1 = x.salary()?;
    let salary2 = y.salary()?;
    Some(salary1 + salary2)
}

enum Job {
    Teacher,
    Scientist,
    Student,
}

impl Job {
    fn salary(&self) -> Option<u32> {
        match self {
            Job::Teacher => Some(50),
            Job::Scientist => Some(70),
            Job::Student => None,
        }
    }
}

fn main() {
    assert_eq!(Some(120), two_salaries(&Job::Teacher, &Job::Scientist));
    assert_eq!(None, two_salaries(&Job::Teacher, &Job::Student));

    println!("Success!");
}

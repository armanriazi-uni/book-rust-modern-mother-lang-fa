#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p other-rc_bin --bin other-rc-main```
///
/// ```cargo doc  --package other-rc_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package other-rc_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
///  `TODO`
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
/// //rust,compile_fail,no_run,ignore
///  `TODO`
///
use std::rc::Rc;
struct Crab {
    name: String,
    age: u32,
    parent: Option<Rc<Crab>>, 
}
fn main() {
    // Create two crabs
    let ferris = Rc::new(Crab {
        name: "Ferris".to_string(),
        age: 5,
        parent: None,
    });
    let cerris = Rc::new(Crab {
        name: "Cerris".to_string(),
        age: 2,
        parent: Some(ferris.clone()), 
    });
    println!("{} is {} years old", ferris.name, ferris.age);
    println!("{} is {} years old", cerris.name, cerris.age);
    if let Some(parent) = &cerris.parent {
        println!("{}'s parent is {}", cerris.name, parent.name);
    }
}

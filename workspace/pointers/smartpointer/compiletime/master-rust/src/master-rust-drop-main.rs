#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-drop_bin --bin master-rust-drop-main```
///
/// ```cargo doc  --package master-rust-drop_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-drop_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
///  When we instantiate any Drop implementing value (any heap allocated type), the Rust compiler inserts drop method calls after every end of scope, after compilation. So, we don't need to manually call drop on these instances. This kind of automatic reclamation based on scope is inspired by the RAII principle of C++.
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
///rust,compile_fail,no_run,ignore
///  `TODO`
struct Character {
    name: String,
}

impl Drop for Character {
    fn drop(&mut self) {
        println!("{} went away", self.name)
    }
}

fn main() {
    let steve = Character {
        name: "Steve".into(),
    };
    let john = Character {
        name: "John".into(),
    };
}

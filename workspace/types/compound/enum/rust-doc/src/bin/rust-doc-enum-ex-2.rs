#![allow(dead_code, unused_variables)]

/// ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-enum_bin --bin rust-doc-enum-ex-2```
///
/// ```cargo doc  --package rust-doc-enum_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-enum_bin```
///
/// ## What
// `refutable_pattern`
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

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    match ferris.species {
        Species::Crab => println!("{} is a crab",ferris.name),
        Species::Octopus => println!("{} is a octopus",ferris.name),
        Species::Fish => println!("{} is a fish",ferris.name),
        Species::Clam => println!("{} is a clam",ferris.name),
    }
}

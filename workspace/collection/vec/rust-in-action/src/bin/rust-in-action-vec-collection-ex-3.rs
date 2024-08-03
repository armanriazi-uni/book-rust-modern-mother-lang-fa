#![allow(dead_code, unused_variables)]

/// rust-in-action-vec-collection-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-vec-collection_bin --bin  rust-in-action-vec-collection-ex-3```
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
/// `unimplemented`
///
/// ## Example
/// //```rust,no_run,compile_fail,ignore


#[derive(Debug)]
struct Individual {
  preference: f32,
  class: String,
}
enum House {
  Occupied(Individual),
  Vacant
}
type Community = Vec<House>;
fn main() {
  let mut town = Community::new();
  let automaton_1 = Individual{ preference: 0.63, class: "+".to_string() };
  let automaton_2 = Individual{ preference: 0.36, class: "o".to_string() } ;
  town.push(House::Occupied(automaton_1));
  town.push(House::Vacant);
  town.push(House::Occupied(automaton_2));

  for house in town.iter() {
    match *house {
      House::Vacant => println!("Available!"),
      House::Occupied(_) => println!("Taken, sorry."),
    }
  }
  for (i, house) in town.iter().enumerate() {
    match *house {
      House::Vacant => println!("No. {} is available!", i),
      House::Occupied(_) => println!("No {} is taken.", i),
    }
  }
  use House::{Vacant, Occupied};
  for (i, house) in town.iter().enumerate() {
    match *house {
      Vacant => println!("No. {} is available!", i),
      Occupied(ref occupant) => println!("No {} is taken by {:?}.", i, occupant.class),
    }
  }
}

#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-in-action-trait-associate-type-and-trait_bin --bin ria-trait-associate-type-and-trait-main```
///
/// ```RUST_BACKTRACE=full cargo test -q -p rust-in-action-trait-associate-type-and-trait_bin --bin ria-trait-associate-type-and-trait-main```
///
/// ```cargo in-action  --package rust-in-action-trait-associate-type-and-trait_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-trait-associate-type-and-trait_bin ```
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

struct Animal {
	age: i32,
}
struct Cat(Animal);
struct Dog(Animal);
struct LoudDog(Animal);
trait Talk {
	fn talk(&self) -> ();
}
impl Talk for Cat {
	fn talk(&self) {
		println!("Meow");
	}
}
impl Talk for Dog {
	fn talk(&self) {
		println!("Woof!");
	}
}
impl Talk for LoudDog {
	fn talk(&self) {
		println!("WOOF!!");
	}
}

fn main() {
	let fluffy = Cat(Animal { age: 4 });
	let max = Dog(Animal { age: 2 });
	let neighbours_dog = LoudDog(Animal { age: 7 });
	fluffy.talk();
	max.talk();
	neighbours_dog.talk();
}

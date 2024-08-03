#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ````RUST_BACKTRACE=full cargo run -q -p master-rust-trait-bound_bin --bin master-rust-trait-bound-main```
///
/// ```cargo doc  --package master-rust-trait-bound_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-trait-bound_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
/// > two kinds of methods within a trait:
/// > self is just a type alias to Self,which refers to the type on which the trait is being implemented
/// > Sample Associated method: pause, Instance methods: play
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

// trait_bounds_intro.rs

struct Game;
struct Enemy;
struct Hero;

trait Loadable {
    fn init(&self);
}

impl Loadable for Enemy {
    fn init(&self) {
        println!("Enemy loaded");
    }
}

impl Loadable for Hero {
    fn init(&self) {
        println!("Hero loaded");
    }
}

impl Game {
    fn load<T: Loadable>(&self, entity: T) {
        entity.init();
    }
}

fn main() {
    let game = Game;
    game.load(Enemy);
    game.load(Hero);
}

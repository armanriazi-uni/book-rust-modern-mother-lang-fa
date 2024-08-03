#![allow(dead_code, unused_variables)]
use std::fmt::Debug;

/// rust-doc-generic-bin-ex-6
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-types-generic_bin --bin rust-doc-generic-bin-ex-6```
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
/// 'nothing'
///
/// ## Example
/// //```rust,compile_fail,ignore

trait CharacterClass { // ①
    type Weapon: Weapon;

    // Add a method to create an instance of the associated type.
    fn create_weapon() -> Self::Weapon;
}

trait Weapon { // ②
    fn attack(&self);
}

struct Warrior; // ③
struct Mage; // ③

struct Sword; //④
struct Staff; // ④

impl Weapon for Sword {
    fn attack(&self) {
        println!("Swinging the sword!");
    }
}

impl Weapon for Staff {
    fn attack(&self) {
        println!("Casting a spell with the staff!");
    }
}

impl CharacterClass for Warrior { // ⑤
    type Weapon = Sword;

    // Implement the method to create a Sword for the Warrior.
    fn create_weapon() -> Self::Weapon {
        Sword
    }
}

impl CharacterClass for Mage { // ⑤
    type Weapon = Staff;

    // Implement the method to create a Staff for the Mage.
    fn create_weapon() -> Self::Weapon {
        Staff
    }
}

fn attack<C: CharacterClass>() { // ⑥
    let weapon = C::create_weapon(); // Create an instance of the associated type.
    weapon.attack();
}

fn main() {
    attack::<Warrior>();
    attack::<Mage>();
}

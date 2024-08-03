#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-struct_bin --bin educative-struct-main```
///
/// ```cargo doc  --package educative-struct_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-struct_bin```
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
/// `unit test`
///
/// ## Example
/// //```rust,compile_fail,ignore
#[derive(PartialEq, Eq, Debug)]
struct Animal {
    name: String,
    color: Color,
}

#[derive(PartialEq, Eq, Debug)]
enum Color {
    Red,
    Yellow,
    Blue,
    Brown,
    White,
}
#[allow(unused_variables, unused_mut)]
fn find_yellow_animal(animals: &[Animal]) -> Option<&Animal> {
    let mut result: Option<&Animal>=None;
    for animal in animals {

        if animal.color == Color::Yellow {
            result = Some(&animal);
            break;
        } else {
             result = None;
        }
    }

    result
}
fn main() {
    let zoo = [
        Animal { name: "Fox".to_owned(), color: Color::Red },
        Animal { name: "Monkey".to_owned(), color: Color::Brown },
        Animal { name: "Polar Bear".to_owned(), color: Color::White },
        Animal { name: "Giraffe".to_owned(), color: Color::Yellow },
        Animal { name: "Canary".to_owned(), color: Color::Red },
        Animal { name: "Dolphin".to_owned(), color: Color::Blue },
    ];

    let animal=find_yellow_animal(&zoo);
    if animal.unwrap().color == Color::Yellow {
            println!("I found a yellow {}. Now I'm done looking.", animal.unwrap().name);
            return;
        } else {
            println!("The {} is not yellow.", animal.unwrap().name);
        }
}

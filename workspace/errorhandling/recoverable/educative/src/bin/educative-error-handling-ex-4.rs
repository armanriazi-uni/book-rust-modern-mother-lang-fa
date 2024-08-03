#![allow(dead_code, unused_variables)]


/// educative-error-handling-ex-4
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p educative-error-handling_bin --bin educative-error-handling-ex-4```
///
/// ```cargo doc  --package educative-error-handling_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-error-handling_bin```
///
/// ## What
/// `error-conversion`
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
///
struct Person {
    name: String,
    age: u32,    // in years
    height: u32, // in centimeters
}
impl Person {
    fn price(&self) -> Result<u32, CantRide> {
        if self.age < 10 {
            return Err(CantRide::TooYoung(self.name.clone()));
        }
        if self.height < 80 {
            return Err(CantRide::TooShort(self.name.clone()));
        }
        Ok(if self.age < 18 {
            5
        } else if self.age < 66 {
            8
        } else {
            7
        })
    }
}
#[derive(Debug)] 
enum CantRide {
    TooShort(String),
    TooYoung(String),
}
fn price_people(people: &[Person]) -> Result<u32, CantRide> {
    let mut price = 0;
    for person in people {
        price += person.price()?;
    }
    Ok(price)
}
fn main() {
    let family = [
        Person { name: "Alice".to_owned(), age: 30, height: 160 },
        Person { name: "Bob".to_owned(), age: 35, height: 170 },
        Person { name: "Charlie".to_owned(), age: 8, height: 100 },
        Person { name: "David".to_owned(), age: 12, height: 75 },
    ];
    println!("{:?}", price_people(&family));
}

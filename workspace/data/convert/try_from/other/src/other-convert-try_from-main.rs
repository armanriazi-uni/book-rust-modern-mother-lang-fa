  #![allow(dead_code, unused_variables, unused_imports)]
 use num::{Float};

/// Main
///
/// # Commands
///
/// ```cargo run -q -p other-convert-try_from_bin --bin other-convert-try_from-main```
///
/// ```cargo test -q -p other-convert-try_from_bin --bin other-convert-try_from-main```
///
/// ```cargo doc  --package other-convert-try_from_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package other-convert-try_from_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` -
///
/// # Return
/// `Ten is less than one hundred.`
///
/// ## Example
/// `nothing`

#[derive(Debug)]
#[allow(dead_code)]
struct Car {
    brand: String
}
impl TryFrom<String> for Car {
    type Error = &'static str;

    fn try_from(brand: String) -> Result<Self, Self::Error> {
        if brand.len() < 1 {
            Err("Invalid brand name")
        } else {
            Ok(Car { brand })
        }
    }
}
impl TryFrom<Car> for String {
    type Error = &'static str;
    fn try_from(car: Car) -> Result<Self, Self::Error> {
        if car.brand == "KIA" || car.brand == "BMW" {
             Err("Not allowed!")
        } else {
            Ok(String::from(car.brand))
        }
    }
}
fn main() {
    let brand_1 = String::from("FORD");
    let car_1 = Car::try_from(brand_1).unwrap();
    println!("car_1: {:?}", car_1); // Car { brand: "FORD" }

    let car_2 = Car::try_from(String::from("")).unwrap_err();
    println!("car_2: {:?}", car_2); // "Invalid brand name"

    let brand_1: Result<String, &'static str> = car_1.try_into();
    println!("brand_1: {:?}", brand_1.unwrap()); // "FORD"

    let car_3 = Car::try_from(String::from("KIA")).unwrap();
    let brand_2: Result<String, &'static str> = car_3.try_into();
    println!("brand_2: {:?}", brand_2.unwrap_err()); // "Not allowed!"
}

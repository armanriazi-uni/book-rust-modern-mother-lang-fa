  #![allow(dead_code, unused_variables, unused_imports)]
 use num::{Float};

/// Main
///
/// # Commands
///
/// ```cargo run -q -p other-convert-into_bin --bin other-convert-into-main```
///
/// ```cargo test -q -p other-convert-into_bin --bin other-convert-into-main```
///
/// ```cargo doc  --package other-convert-into_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package other-convert-into_bin```
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
impl From<String> for Car {
    fn from(brand: String) -> Self {
        Car { brand }
    }
}
impl From<Car> for String {
   fn from(car: Car) -> Self {
        String::from(car.brand)
    }
}
fn main() {
    let brand_1 = String::from("FORD");
    let car_1 = Car::from(brand_1); 
    println!("car_1: {:?}", car_1); // car_1: Car { brand: "FORD" }
    let car_1_brand: String = car_1.into();
    println!("car_1_brand: {:?}", car_1_brand); // car_1_brand: "FORD"
}

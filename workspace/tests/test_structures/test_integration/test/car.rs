#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo tarpaulin  -- -q -p test_integration car -nocapture```
/// 
/// ```cargo test -q -p test_integration  car -- --nocapture```
///
/// ```cargo doc  --package test_integration --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc --package test_integration```
///
/// ## What
/// `Test Integration Structure`
///
/// ## How
/// `Test functions have to return a type that implements the std::process::Termination trait (the main function has this same restriction). Practically, this means that the returns either a unit () or a Result. If the test function returns a Result type, the test will pass if the Result is Ok, and fail if the Result is Err. Returning a result is useful when testing code that may ail, but we want to continue testing or setting up the test code includes potentially panicking code. We'll get to use the convenient ? operator to propagate the errors and avoid verbose matching.`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// `nothing`
///


#[test]
fn test_public_car() {
    let car = lib::car::PublicCar {
        owner: "Ylvis Stonehenge".to_string(),
        license: "ABC123".to_string(),
        model: "Civic".to_string(),
        make: "Honda".to_string(),
        year: 2013,
    };
    assert_eq!(car.owner, "Ylvis Stonehenge");
    assert_eq!(car.license, "ABC123");
    assert_eq!(car.model, "Civic");
    assert_eq!(car.make, "Honda");
    assert_eq!(car.year, 2013);
}

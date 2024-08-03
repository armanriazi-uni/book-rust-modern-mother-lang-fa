#![allow(dead_code, unused_variables)]



/// Main
///
/// ## Commands
///
/// ```cargo test -q -p fixture_test --bin test_fixture-ex-1```
///
/// ```cargo doc  --package fixture_test --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package fixture_test```
///
/// ## What
/// `List of values (rust expressions) `
///
/// ## How
/// `In this cases [rstest] give you the ability to define a list of values (rust expressions) to use for an arguments.`
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

#[cfg(test)]
mod fixt {

    struct TestFixture {
        pub name : String
    }

    fn Setup<F>( tf: F ) where F: Fn (&TestFixture) {
        let o = TestFixture { name: String::from ("Initialized") };
        tf (&o);
    }

    #[test]
    fn make_sure_foo_works() {
        Setup(| x | {
            assert_eq!(x.name, "Initialized");
        } );

        // Do a similar thing here with a teardown
    }
}

fn main() {

    println!("{}", "Printed");
}


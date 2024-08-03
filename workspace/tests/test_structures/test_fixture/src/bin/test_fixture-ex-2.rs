#![allow(dead_code, unused_variables)]



/// Main
///
/// ## Commands
///
/// ```cargo test -q -p fixture_test --bin test_fixture-ex-2```
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


macro_rules! unit_tests {
    ($( fn $name:ident($fixt:ident : $ftype:ty) $body:block )*) => (
        $(
            #[test]
            fn $name() {
                let mut $fixt = <$ftype as Fixture>::setup();
                $body
                $fixt.teardown();
            }
        )*
    )
}

trait Fixture {
    fn setup() -> Self;
    fn teardown(self);
}

struct FooTestFixture<T> {
    name: String,
    num: T,
}
impl FooTestFixture<i32> {
    fn add_some(&mut self) {
        self.num += 10;
    }
}
impl<T: Default> Fixture for FooTestFixture<T> {
    fn setup() -> FooTestFixture<T> {
        FooTestFixture {
            name: String::from("Initialised"),
            num: Default::default(),
        }
    }
    fn teardown(mut self) {
        self.name = "".to_string();
    }
}

unit_tests!{
    fn some_test_name(f: FooTestFixture<()>) {
        assert_eq!(f.name, "Initialised");
        f.name = "Foo".to_owned();
        assert_eq!(f.name, "Foo");
    }

    fn another_test(g: FooTestFixture<i32>) {
        assert_eq!(g.num, 0);
        g.add_some();
        assert_eq!(g.num, 10);
    }
}

fn main(){
    unimplemented!();
}
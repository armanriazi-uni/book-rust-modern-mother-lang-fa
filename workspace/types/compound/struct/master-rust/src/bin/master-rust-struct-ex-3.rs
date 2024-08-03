#![allow(dead_code, unused_variables)]

/// master-rust-struct-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-struct_bin --bin master-rust-struct-ex-3```
///
/// ```cargo doc  --package master-rust-struct_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-struct_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `the compiler thinks that we want to match against Person(&String) but the a value is actually Person(String). So, in this case ref has to be used to destructure it as a reference. To make it compile, we change it accordingly to Person(ref name) on the left-hand side.`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `assert:true`
///
/// ## Example
/// //```rust

// enum Foo {
//     One, Two, Three
// }

// enum Bar(Foo);

// struct Dummy {
//     inner: Bar
// }

// struct ComplexStruct {
//     obj: Dummy
// }

// fn get_complex_struct() -> ComplexStruct {
//     ComplexStruct {
//         obj: Dummy { inner: Bar(Foo::Three) }
//     }
// }

fn main() {
    //let a = get_complex_struct();
    unimplemented!();
}

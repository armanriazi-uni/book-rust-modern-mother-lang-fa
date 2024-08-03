#![allow(dead_code, unused_variables)]

/// rust-doc-trait-associate-type-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-trait-associate-type-and-trait_bin --bin  rust-doc-trait-associate-type-ex-4```
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
/// `unimplemented`
///
/// ## Example

struct Struct;
struct GenericStruct<const ID: i32>;

// impl Struct {
//     // Definition not immediately evaluated
//     const PANIC: () = panic!("compile-time panic");
// }

impl<const ID: i32> GenericStruct<ID> {
    // Definition not immediately evaluated
    const NON_ZERO: () = if ID == 0 {
        panic!("contradiction")
    };
}

fn main() {
    // Referencing Struct::PANIC causes compilation error
    //let _ = Struct::PANIC;

    // Fine, ID is not 0
    let _ = GenericStruct::<1>::NON_ZERO;

    // Compilation error from evaluating NON_ZERO with ID=0
    //let _ = GenericStruct::<0>::NON_ZERO;
}

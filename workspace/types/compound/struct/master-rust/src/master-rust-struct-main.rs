#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-struct_bin --bin master-rust-struct-main```
///
/// ```cargo doc  --package master-rust-struct_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-struct_bin```
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
/// `assert:true`
///
/// ## Example
/// //```rust,compile_fail,ignore
// destructure_struct.rs

enum Food {
    Pizza,
    Salad
}

enum PaymentMode {
    Bitcoin,
    Credit
}

struct Order {
    count: u8,
    item: Food,
    payment: PaymentMode
}

fn main() {
    let food_order = Order { count: 2,
                             item: Food::Salad,
                             payment: PaymentMode::Credit };

    // let can pattern match inner fields into new variables
    let Order { count, item, .. } = food_order;
}

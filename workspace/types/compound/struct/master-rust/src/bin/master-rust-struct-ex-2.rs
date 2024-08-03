#![allow(dead_code, unused_variables)]

/// master-rust-struct-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-struct_bin --bin master-rust-struct-ex-2```
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
// destructure_func_param.rs

struct Container {
    items_count: u32
}

fn increment_item(Container {mut items_count}: &mut Container) {
    items_count += 1;
}

fn calculate_cost(Container {items_count}: &Container) -> u32 {
    let rate = 67;
    rate * items_count
}

fn main() {
    let mut container = Container {
        items_count: 10
    };

    increment_item(&mut container);
    let total_cost = calculate_cost(&container);
    println!("Total cost: {}", total_cost);
}

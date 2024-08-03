#![allow(dead_code, unused_variables)]

/// educative-dangling-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p educative-dangle_bin --bin  educative-dangling-bin-ex-1```
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
///```rust,no_run,compile_fail,ignore
///
/// fn get_greater_cost(product1: &Product, product2: &Product) -> &Product { //Exposed Error
///     if product1.cost > product2.cost {
///         product1
///     } else {
///         product2
///     }
/// }
///
/// fn main() {
///     let product1 = Product{name: "shoes".to_string(), cost: 100.0};
///     let product2 = Product{name: "hat".to_string(), cost: 90.0};
///     println!("The product with the high cost is: {:#?}", get_greater_cost(&product1, &product2));
/// }
/// `Output`
/// error[E0106]: missing lifetime specifier

#[derive(Debug)]
struct Product {
    name: String,
    cost: f64
}
fn price(product: Product) -> f64 {
    product.cost * 1.30
}
fn discount(product: Product) -> f64 {
    price(product) / 1.10
}
fn get_high_cost<'a>(product1: &'a Product, product2: &'a Product) -> &'a Product {
    if product1.cost > product2.cost {
        product1
    } else {
        product2
    }
}
fn main() {
    let product1 = Product{name: "shoes".to_string(), cost: 100.0};
    let product2 = Product{name: "hat".to_string(), cost: 90.0};
    println!("The product with the high cost is: {:#?}", get_high_cost(&product1, &product2));
}

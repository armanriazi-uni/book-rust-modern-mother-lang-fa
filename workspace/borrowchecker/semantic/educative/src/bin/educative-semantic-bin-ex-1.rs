#![allow(dead_code, unused_variables)]

/// educative-semantic-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p educative-semantic_bin  --bin  educative-semantic-bin-ex-1```
///
/// ## What
/// `/// `primitives do not requiring impl Copy trait``
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// ```rust,no_run,compile_fail,ignore
/// struct Product {
///     name: String,
///     cost: f64
/// }
///
/// fn price(product: Product) -> f64 {
///     product.cost * 1.30
/// }
///
/// fn discount(product: Product) -> f64 {
///     price(product) / 1.10
/// }
///
/// fn main() {
///     let hat = Product {name: "Hat".to_string(), cost: 100.0};
///     println!("The price of product: {}, is {:.2}", hat.name, price(hat)); //Error Expose
/// }
///
/// ```
/// `Output`
/// error[E0505]: cannot move out of `hat` because it is borrowed

struct Product {
    name: String,
    cost: f64
}
fn price(product: &Product) -> f64 {
    product.cost * 1.30
}
fn discount(product: &Product) -> f64 {
    price(product) / 1.10
}
fn main() {
    let hat = Product {name: "Hat".to_string(), cost: 100.0};
    println!("The price of product: {}, is {:.2}", &hat.name, price(&hat));
}

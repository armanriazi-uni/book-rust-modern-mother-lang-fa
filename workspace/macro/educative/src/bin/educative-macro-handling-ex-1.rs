#![allow(dead_code, unused_variables)]

/// educative-macro-handling-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p educative-macro-handling_bin --bin  educative-macro-handling-ex-1```
///
/// ## What
/// The macro above takes a list of parameters in $( $x:expr ),*, then it uses them in a loop to sum all the costs of the products provided.
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
/// //```rust,no_run,compile_fail,ignore

macro_rules! sum_amount {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_num = 0.0;
            $(
                temp_num += $x.cost;
            )*
            temp_num
        }
    };
}

struct Product {
    cost: f64
}

fn main() {
    let shoes = Product { cost: 10.5 };
    let hat = Product { cost: 15.8 };
    let pants = Product { cost: 22.0 };
    println!("The total cost is: {}", sum_amount!(shoes, hat, pants));
}

#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p educative-macro-handling_bin --bin educative-macro-handling-main```
///
/// ```cargo doc  --package educative-macro-handling_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-macro-handling_bin ```
///
/// ## What
/// `macro-share behavior`
///
/// ## How
/// The macro above takes a list of parameters in $( $x:expr ),*, then it uses them in a loop to sum all the costs of the products provided.
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// ```rust
/// This is my sale: Sale {
///     product_details: [
///         ProductDetail {
///             product: Product {
///                 name: "shoes",
///                 cost: 45.65,
///             },
///             discount: None,
///             tax: 21.0,
///             price: Some(
///                 120.0,
///             ),
///         },
///     ],
///     date: "10/12/2020",
///     subtotal: Some(
///         120.0,
///     ),
///     total_tax: 21.0,
///     total: Some(
///         141.0,
///     ),
/// }
/// ```

type Date = String;

#[allow(dead_code)]
#[derive(Debug)]
struct Product {
    name: String,
    cost: f64
}

#[allow(dead_code)]
#[derive(Debug)]
struct ProductDetail {
    product: Product,
    discount: Option<f64>,
    tax: f64,
    price: Option<f64>
}

///  create three models that are similar but have different business logic.
macro_rules! create_model {
    ($name:ident) => {
        #[allow(unused_variables)]
        #[allow(dead_code)]
        #[derive(Debug)]
        struct $name {
            product_details: Vec<ProductDetail>,
            date: Date,
            subtotal: Option<f64>,
            total_tax: f64,
            total: Option<f64>
        }
    }
}

create_model!(Sale);
// create_model!(Purchase);
// create_model!(Budget);

/// we make another macro call calculate_total that is used to create three similar traits for the structs that share a similar behavior.
macro_rules! calculate_total {
    ($name:ident, $model:ty) => {
        trait $name {
            fn calculate_subtotal(data_model: &$model) -> f64 {
                data_model.product_details.iter().map(|product_detail|
                    product_detail.price.unwrap_or(0.0) -
                        product_detail.discount.unwrap_or(0.0)
                ).sum()
            }

            fn calculate_total(data_model: &$model) -> f64 {
                data_model.subtotal.unwrap_or(0.0) + data_model.total_tax
            }
        }

        impl $name for $model {}
    }
}

calculate_total!(CalculationsSale, Sale);
// calculate_total!(CalculationsPurchase, Purchase);
// calculate_total!(CalculationsBudget, Budget);

fn main() {
    let shoes = Product {
        name: "shoes".to_string(),
        cost: 45.65
    };
    let product_detail = ProductDetail {
        product: shoes,
        discount: None,
        tax: 21.0,
        price: Some(120.0)
    };
    let mut sale = Sale {
        product_details: vec![product_detail],
        date: "10/12/2020".to_string(),
        subtotal: None,
        total_tax: 21.0,
        total: None
    };
    sale.subtotal = Some(Sale::calculate_subtotal(&sale));
    sale.total = Some(Sale::calculate_total(&sale));
    println!("This is my sale: {:#?}", sale);
}

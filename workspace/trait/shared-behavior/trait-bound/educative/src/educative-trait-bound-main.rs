#![allow(dead_code, unused_variables)]

use std::fmt::Debug;

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p educative-trait-bound_bin --bin educative-trait-bound-main```
///
/// ```cargo doc  --package educative-trait-bound_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-trait-bound_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
/// `Notice`
/// Pay attention to `->Finance` enum. it is works as the same as `->(impl Invoicing + Debug)`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// fn notify<T: Invoicing>(user_name: String, invoicing: T) {
///     println!(
///         "Client: {}, has an invoice total: {} and was notified!",
///         user_name,
///         invoicing.generates_total()
///     );
/// }

#[derive(Debug)]
struct Sale {
    subtotal: f64,
    tax: f64
}
#[derive(Debug)]
struct Purchase {
    subtotal: f64,
    tax: f64
}
#[derive(Debug)]
enum Finance {
    Income(Sale),
    Expenditure(Purchase)
}


// Here we use the Invoicing trait to share the tax, subtotal and generates_total methods
// between Sale and Purchase structs.
trait Invoicing {
    fn tax(&self) -> f64;
    fn subtotal(&self) -> f64;
    fn generates_total(&self) -> f64 {
        self.tax() + self.subtotal()
    }
}

impl Invoicing for Sale {
    fn tax(&self) -> f64 {
        self.tax
    }
    fn subtotal(&self) -> f64 {
        self.subtotal
    }
}
impl Invoicing for Purchase {
    fn tax(&self) -> f64 {
        self.tax
    }
    fn subtotal(&self) -> f64 {
        self.subtotal
    }
}
fn print_invoice(invoice_type: &str) -> Finance {
    if invoice_type == "Sale" {
        Finance::Income(Sale { subtotal: 12.34, tax: 21.0 })
    } else {
        Finance::Expenditure(Purchase { subtotal: 65.23, tax: 16.7 })
    }
}

/// The invoicing argument is able to accept any struct that implements
/// the Invoicing and Debug traits.
fn notify(user_name: String, invoicing: (impl Invoicing + Debug)) {
    println!(
        "Client: {}, has an invoice total: {} and was notified!",
        user_name,
        invoicing.generates_total()
    );
}
fn print_sale() -> impl Invoicing + Debug {
    Sale { subtotal: 98.75, tax: 21.5 }
}

fn print_purchase() -> impl Invoicing + Debug {
    Purchase { subtotal: 65.34, tax: 16.9 }
}

fn main() {
    let sale = Sale {subtotal: 100.0, tax: 21.0};
    notify("Michael".to_string(), sale);
    //
    println!("Print sale: {:#?}", print_sale());
    println!("Print purchase: {:#?}", print_purchase());
    //
    println!("This is a sale: {:#?}", print_invoice("Sale"));
    println!("This is a purchase: {:#?}", print_invoice("Purchase"));
}

#![allow(dead_code, unused_variables)]

/// master-rust-semantic-bin-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-semantic_bin --bin  master-rust-semantic-bin-ex-3```
///
/// ## What
/// `/// `primitives do not requiring impl Copy trait``
///
/// ## How
/// `The ref keyword is a keyword that can match items by taking a reference to them instead of capturing them by value`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `I got cake`
/// `Bag { food: Cake }`
///
/// ## Example
///```rust,no_run,compile_fail,ignore
/// fn get_a_borrowed_value() -> &u8 {
///     let x = 1;
///     &x
/// }

/// fn main() {
///     let value = get_a_borrowed_value();
/// }

///```

#[derive(Debug)]
struct Items(u32);
fn main() {
    let items = Items(2);
    let items_ptr = &items;
    let ref items_ref = items;
    assert_eq!(items_ptr as *const Items, items_ref as *const Items);
    let mut a = Items(20);   
    {
        let ref mut b = a; 
        b.0 += 25;
    }
    println!("{:?}", items);
    println!("{:?}", a);   
}

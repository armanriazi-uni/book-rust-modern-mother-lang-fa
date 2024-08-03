#![allow(dead_code, unused_variables)]
use lib::linkedlist::*;


/// Main
///
/// ## Commands
///
/// 
/// ```cargo test -q -p packtpub-smartpointer-ref-cell-linkedlist_lib  linkedlist -- --nocapture```
///
/// ```cargo doc  --package packtpub-smartpointer-ref-cell-linkedlist_lib --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc --package packtpub-smartpointer-ref-cell-linkedlist_lib```
///
/// ## What
/// `Test Integration Structure`
///
/// ## How
/// `Test functions have to return a type that implements the std::process::Termination trait (the main function has this same restriction). Practically, this means that the returns either a unit () or a Result. If the test function returns a Result type, the test will pass if the Result is Ok, and fail if the Result is Err. Returning a result is useful when testing code that may ail, but we want to continue testing or setting up the test code includes potentially panicking code. We'll get to use the convenient ? operator to propagate the errors and avoid verbose matching.`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// `nothing`
///

#[test]
fn test_list_insert_10k_items() {
    let mut list = List::new_empty();
    for _ in 0..10_000 {
        list.append(100);
    }
    assert_eq!(list.length, 10_000);
}
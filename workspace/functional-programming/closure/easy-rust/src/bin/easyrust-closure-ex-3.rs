#![allow(dead_code, unused_variables)]
/// easyrust-closure-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p easyrust-closure_bin --bin easyrust-closure-ex-3```
///
/// ```cargo doc  --package easyrust-closure_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package easyrust-closure_bin ```
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
/// `nothing`
///
/// ## Example
///  `TODO`
///
fn do_something<F>(f: F)
where
    F: FnOnce(),
{
    f();
}
fn main() {
    let some_vec = vec![9, 8, 10];
    do_something(|| {
        some_vec
            .into_iter()
            .for_each(|x| println!("The number is: {}", x));
    })
}

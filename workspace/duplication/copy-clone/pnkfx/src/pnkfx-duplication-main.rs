#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p pnkfx-duplication_bin --bin pnkfx-duplication-main```
///
/// ```cargo doc  --package pnkfx-duplication_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package pnkfx-duplication_bin ```
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
///
// Example 3: mutation

#[derive(Copy, Clone)]
struct Thing {
    label: char,
    count: i32,
}

pub fn main() {
    let i = 5;

    let t_a = Thing { label: 'a', count: i };

    let mut t_b = t_a;
    //  ^       ^~~~~ initialize `t_b` as the value of `t_a`.
    //  |
    //  ^~~ the `mut` keyword marks a local binding that we plan to "mutate"
    //  i.e. modify in some fashion that requires exclusive access.

    // Mutability is inherited: If you have `mut`-access to `t`, then you
    // have `mut` access to the fields of `t`.

    t_b.label = 'b';
    //        ^~~~~ this is an example of a mutation of `t_b`;

    print!("t_a initially ");
    print_thing(t_a);
    print!("t_b initially ");
    print_thing(t_b);

    try_to_change(t_b);

    print!("t_a post ttc: ");
    print_thing(t_a);
    print!("t_b post ttc: ");
    print_thing(t_b);
}

fn print_thing(x: Thing) {
    println!("the count of {} is {}", x.label, x.count);
}

fn try_to_change(x: Thing) {
    // EXERCISE: increment the count of `x` within this function,
    // so that the print-out below proves that your added code
    // did indeed increment the count.
    print!("    x in ttc: ");
    print_thing(x);
}

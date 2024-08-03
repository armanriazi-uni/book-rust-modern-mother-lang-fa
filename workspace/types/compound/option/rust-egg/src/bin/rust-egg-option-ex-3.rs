#![allow(dead_code, unused_variables)]

/// rust-egg-option-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-option_bin --bin rust-egg-option-ex-3```
///
/// ```cargo doc  --package rust-egg-option_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-egg-option_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your option/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  // ```rust,compile_fail,ignore
// The adult has seen it all, and can handle any drink well.
// All drinks are handled explicitly using `match`.
fn give_adult(drink: Option<&str>) {
    // Specify a course of action for each case.
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner)   => println!("{}? How nice.", inner),
        None          => println!("No drink? Oh well."),
    }
}

// Others will `panic` before drinking sugary drinks.
// All drinks are handled implicitly using `unwrap`.
fn drink(drink: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    let inside = drink.unwrap();
    if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let water  = Some("water");
    let lemonade = Some("lemonade");
    let void  = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("yyy");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}

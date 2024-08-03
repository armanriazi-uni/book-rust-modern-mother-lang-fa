#![allow(dead_code, unused_variables)]
/// Main
///
/// ## Commands
///

/// ```cargo run -q -p rd-macro-feature_bin --bin rd-macro-feature-main --message-format=json-render-diagnostics```
///
/// ```cargo doc  --package rd-macro-feature_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rd-macro-feature_bin```
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
/// ```
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate features;

features! {
    pub mod feature {
        const Alpha = 0b00000001,
        const Beta = 0b00000010
    }
}

fn main() {
    assert_eq!(false, feature::is_enabled(feature::Alpha));
    assert_eq!(false, feature::is_enabled(feature::Beta));

    feature::enable(feature::Beta);
    assert_eq!(false, feature::is_enabled(feature::Alpha));
    assert_eq!(true, feature::is_enabled(feature::Beta));
}
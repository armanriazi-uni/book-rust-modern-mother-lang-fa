#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p educative-trait-associate-type-and-trait_bin --bin educative-trait-associate-type-and-trait-main```
///
/// ```cargo in-action  --package educative-trait-associate-type-and-trait_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-trait-associate-type-and-trait_bin ```
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
fn main() {
    let speed = Kmh { value: 90 };
    let distance = speed.in_three_hours();
    println!("At {:?}, you will travel {:?} in 3 hours", speed, distance);
}
trait InThreeHours {
    fn in_three_hours(&self) -> Km;
}

impl InThreeHours for Kmh {
    fn in_three_hours(&self) -> Km {
        Km { value: self.value * 3 }
    }
}
#[derive(Debug, Clone, Copy)]
struct Kmh {
    value: u32
}
impl Kmh {
    fn in_three_hours(&self) -> Km {
        Km { value: self.value * 2 }
    }
}
#[derive(Debug, Clone, Copy)]
struct Km {
    value: u32
}

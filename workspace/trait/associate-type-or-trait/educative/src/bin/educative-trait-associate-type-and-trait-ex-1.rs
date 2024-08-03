#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p educative-trait-associate-type-and-trait_bin --bin educative-trait-associate-type-and-trait-ex-1```
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
    let speed = Mph { value: 90 };
    let distance = speed.in_three_hours();
    println!("At {:?}, you will travel {:?} in 3 hours", speed, distance);
}
trait InThreeHours {
    type Distance;
    fn in_three_hours(&self) -> Self::Distance;
}
#[derive(Debug, Clone, Copy)]
struct Kmh {
    value: u32
}

#[derive(Debug, Clone, Copy)]
struct Km {
    value: u32
}
#[derive(Debug, Clone, Copy)]
struct Mph {
    value: u32
}
impl InThreeHours for Mph {
    type Distance = Miles;
    fn in_three_hours(&self) ->  Self::Distance {
        Miles { value: self.value * 3 }
    }
}

#[derive(Debug, Clone, Copy)]
struct Miles {
    value: u32
}

impl InThreeHours for Kmh {
    type Distance = Km;
    fn in_three_hours(&self) ->  Self::Distance {
        Km { value: self.value * 2 }
    }
}


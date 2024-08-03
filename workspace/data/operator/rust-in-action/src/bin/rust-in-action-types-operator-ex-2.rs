#![allow(dead_code, unused_variables)]

/// rust-in-action-types-operator-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-types-operator_bin --bin rust-in-action-types-operator-ex-2```
///
/// ```cargo doc  --package rust-in-action-types-operator_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-types-radix_bin```
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
#[derive(Debug)]
struct Score(isize);

impl std::ops::AddAssign for Score {
    fn add_assign(&mut self, right_hand: Score) {
        self.0 += right_hand.0;
    }
}

fn main() {
    let mut scores = vec![
        Score(1),
        Score(3),
        Score(5)
    ];
    println!("initial: {:?}", scores);

    for mut score in &mut scores {
        *score += Score(1);
    }
    println!("imperative: {:?}", scores);

    //scores.iter_mut().map(|score| *score += Score(1)).collect();
    println!("higher-order: {:?}", scores);
}

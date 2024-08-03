#![allow(dead_code, unused_variables)]

/// rust-doc-state-behaviora-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-state-behavioral_bin --bin rust-doc-state-behavioral-ex-2```
///
/// ```cargo doc  --package rust-doc-state-behavioral_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-state-behavioral_bin```
///
/// ## What
///If we were to create an alternative implementation that didn’t use the state pattern, we might instead use match expressions in the methods on Post or even in the main code that checks the state of the post and changes behavior in those places. That would mean we would have to look in several places to understand all the implications of a post being in the published state! This would only increase the more states we added: each of those match expressions would need another arm.
///
///With the state pattern, the Post methods and the places we use Post don’t need match expressions, and to add a new state, we would only need to add a new struct and implement the trait methods on that one struct.
///
/// ## How
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
/// //```rust,compile_fail,no_run,ignore
enum TrafficLightState {
    Red,
    Yellow,
    Green,
}
trait TrafficLightStateTrait {
    fn next_state(&self) -> TrafficLightState;
}
impl TrafficLightStateTrait for TrafficLightState {
    fn next_state(&self) -> TrafficLightState {
        match *self {
            TrafficLightState::Red => TrafficLightState::Green,
            TrafficLightState::Yellow => TrafficLightState::Red,
            TrafficLightState::Green => TrafficLightState::Yellow,
        }
    }
}
struct TrafficLight {
    state: TrafficLightState,
}
impl TrafficLight {
    fn new() -> Self {
        TrafficLight { state: TrafficLightState::Red }
    }

    fn change_state(&mut self) {
        self.state = self.state.next_state();
    }
}
fn main() {
    let mut traffic_light = TrafficLight::new();

    println!("Initial state: {}", match traffic_light.state {
        TrafficLightState::Red => "Red",
        TrafficLightState::Yellow => "Yellow",
        TrafficLightState::Green => "Green",
    });

    for _ in 0..10 {
        traffic_light.change_state();
        println!("Next state: {}", match traffic_light.state {
            TrafficLightState::Red => "Red",
            TrafficLightState::Yellow => "Yellow",
            TrafficLightState::Green => "Green",
        });
    }
}

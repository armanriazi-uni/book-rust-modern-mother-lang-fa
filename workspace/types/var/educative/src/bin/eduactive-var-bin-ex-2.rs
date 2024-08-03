#![allow(dead_code, unused_variables)]

/// eduactive-var-bin-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p eduactive-var_bin --bin eduactive-var-bin-ex-2```
///
/// ## What
/// `dewrapperure`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your wrapper/func name]
///
/// # Return
/// `assert:true`
///
/// ## Example
/// //```rust,compile_fail,ignore
///
fn main() {
    let apples = 10;
    println!("Before the block, I have {} apples", apples);
    {
        let apples = apples + 5;
        println!("Inside the block, I have {} apples", apples);
    }
    println!("After the block, I have {} apples", apples);
}

#![allow(dead_code, unused_variables)]

/// eduactive-var-bin-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p eduactive-var_bin --bin eduactive-var-bin-ex-1```
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
    let apples = 10; // apples is 10
    println!("apples == {}", apples);
    {
        let apples = 20; // shadow apples in this scope
        println!("apples == {}", apples);
    }
    // That block's scope is done
    // Now our original apples is back in scope
    // What do you think this will output?
    println!("apples == {}", apples);
}

#![allow(dead_code, unused_variables)]

/// eduactive-var-bin-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p eduactive-var_bin --bin eduactive-var-bin-ex-3```
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
  let outer_variable = 112;
  { // start of code block
        let inner_variable = 213;
        println!("block variable: {}", inner_variable);
        let outer_variable = 117;
        println!("block variable outer: {}", outer_variable);
  } // end of code block
    println!("outer variable: {}", outer_variable);
  }

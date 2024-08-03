#![allow(dead_code, unused_variables)]


/// rust-doc-smartpointer-box-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-smartpointer-box_bin --bin  rust-doc-smartpointer-box-ex-4```
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
/// //``rust,no_run,compile_fail,ignore
fn modifier1(mut ptr: Box<String>) -> Box<String> {
    println!("In modifier1...");
    println!("Ptr points to {:p}, and value is {}", ptr, *ptr);
    *ptr = ptr.to_uppercase();
    println!("Exit modifier1...");
    ptr
}
fn modifier2(ptr: &mut Box<String>) {
    println!("In modifier2...");
    println!("Ptr points to {:p}, and value is {}", ptr, *ptr);
    println!("Ptr points to {:p}, and value is {}", *ptr, **ptr);
    **ptr = "another".to_uppercase();
    **ptr = ptr.to_uppercase(); 
    println!("Exit modifier2...");
}
fn main() {
    let mut answer = Box::new("Hello World".to_string());
    answer = modifier1(answer);
    println!("called modifier1(): {} length: {}", answer, answer.len());
    let mut answer = Box::new("Hello World".to_string());
    modifier2(&mut answer);
    println!("called modifier2(): {} length: {}", answer, answer.len());
}

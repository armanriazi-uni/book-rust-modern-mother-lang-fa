#![allow(dead_code, unused_variables)]
// use std::fs::File;
// use std::io::{Read,Write,copy};

/// rust-in-action-file-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-file_bin --bin rust-in-action-file-ex-1```
///
/// ```cargo doc  --package rust-in-action-file_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-file_bin```
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
/// `100`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// // ```compile_fail,ignore
///
///fn main()->std::io::Result<()> {
///    let mut s = String::new();
///
///    let mut f = File::open("data/../from.md")?;
///    f.read_to_string(&mut s)?;
///
///    println!("{}",s);
///
///
///    let mut t = File::create("data/../to.md")?;
///
///    //copy(&mut f,&mut t)?;
///
///    t.write_all(&s.into_bytes())?;
///
///    Ok(())
///}
/// ```
fn main(){
    unimplemented!()
}

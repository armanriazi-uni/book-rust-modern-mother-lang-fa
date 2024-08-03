#![allow(dead_code, unused_variables)]


use std::fs::File;
use std::io::Read;



/// rust-doc-error-handling-ex-6
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-error-handling_bin --bin  rust-doc-error-handling-ex-6```
///
/// ## What
/// `Propagating errors`
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

//use error_chain::error_chain;
// error_chain!{
//     foreign_links {
//         Io(std::io::Error);
//         ParseInt(::std::num::ParseIntError);
//     }
// }

// fn read_uptime() -> Result<u64> {
//     let mut uptime = String::new();
//     File::open("/proc/uptime")?.read_to_string(&mut uptime)?;

//     Ok(uptime
//         .split('.')
//         .next()
//         .ok_or("Cannot parse uptime data")?
//         .parse()?)
// }

fn main() {
    unimplemented!();
    // match read_uptime() {
    //     Ok(uptime) => println!("uptime: {} seconds", uptime),
    //     Err(err) => eprintln!("error: {}", err),
    // };
}

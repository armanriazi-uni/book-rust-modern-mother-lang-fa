#![allow(dead_code, unused_variables)]



/// rust-doc-trait-associate-type-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-trait-associate-type-and-trait_bin --bin  rust-doc-trait-associate-type-ex-2```
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

#[derive(Debug)]
struct File;

trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(100)
    }
}

fn main() {
    /*
    let f = File{};
    let mut buffer = vec!();
    let n_bytes = f.read(&mut buffer).unwrap(); //error: reading zero byte data to `Vec`
    println!("{} byte(s) read from {:?}", n_bytes, f);
    */
    unimplemented!();
}

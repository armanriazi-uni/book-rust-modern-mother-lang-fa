#![allow(dead_code, unused_variables)]
use rand::prelude::*;                      // <1>


/// rust-in-action-error-handling-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-error-handling_bin --bin  rust-in-action-error-handling-ex-1```
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

/// thread_rng() creates a thread-local random number generator; gen_ratio(n, m) returns a Boolean value with an n/m probability.
/// Helper function that triggers sporadic errors


#[derive(Debug)]
struct File {
  name: String,
  data: Vec<u8>,
}

fn one_in(denominator: u32) -> bool {      // <2>
  thread_rng().gen_ratio(1, denominator)   // <3>
}

impl File {
  fn new(name: &str) -> File {
    File {
      name: String::from(name),
	    data: Vec::new()
    }                                      // <4>
  }

  fn new_with_data(name: &str, data: &Vec<u8>) -> File {
    let mut f = File::new(name);
    f.data = data.clone();
    f
  }

  fn read(
    self: &File,
    save_to: &mut Vec<u8>,
  ) -> Result<usize, String> {             // <5>
    let mut tmp = self.data.clone();
    let read_length = tmp.len();
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    Ok(read_length)                        // <6>
  }
}

fn open(f: File) -> Result<File, String> {
  if one_in(10_000) {                              // <7>
    let err_msg = String::from("Permission denied");
    return Err(err_msg);
  }
  Ok(f)
}

fn close(f: File) -> Result<File, String> {
  if one_in(100_000) {                             // <8>
    let err_msg = String::from("Interrupted by signal!");
    return Err(err_msg);
  }
  Ok(f)
}

fn main() {
  let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
  let mut f4 = File::new_with_data("4.txt", &f4_data);
  let mut buffer: Vec<u8> = vec![];
  f4 = open(f4).unwrap();                          // <9>
  let f4_length = f4.read(&mut buffer).unwrap();   // <9>
  f4 = close(f4).unwrap();                         // <9>
  let text = String::from_utf8_lossy(&buffer);
  println!("{:?}", f4);
  println!("{} is {} bytes long", &f4.name, f4_length);
  println!("{}", text);
}

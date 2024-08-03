#![allow(dead_code, unused_variables)]


/// rust-in-action-file-ex-5
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-file_bin --bin rust-in-action-file-ex-5```
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
/// `nothing`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// // ```compile_fail,ignore
#[derive(Debug)] // <1>
struct File {
  name: String,
  data: Vec<u8>, // <2>
  state: &'static str,
}

type FileMode = &'static str;

const OPEN: FileMode = "open";
const CLOSED: FileMode = "closed";

fn open(f: &mut File) -> bool {
    f.state = OPEN;
    true // <3> let's assume for the moment that this always succeeds
}

fn close(f: &mut File) -> bool {
    f.state = CLOSED;
    true // <3>
}

fn read(f: &File) -> (usize, Vec<u8>) {
    (f.data.len(), f.data.clone())
}

fn main() {
  let mut f2 = File {
    name: String::from("2.txt"), // <3>
    data: vec![],
    state: CLOSED,
  };

  let f2_name = &f2.name.clone(); // <5>

  open(&mut f2);
  let (f2_length, _) = read(&f2);
  close(&mut f2);

  println!("{:?}", f2);
  println!("{} is {} bytes long", f2_name, f2_length);
}

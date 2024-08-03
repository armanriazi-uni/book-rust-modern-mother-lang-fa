#![allow(dead_code, unused_variables)]
use std::fmt;
use std::fmt::{Display};
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-enum_bin --bin rust-in-action-enum-main```
///
/// ```cargo doc --package rust-in-action-enum_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc --package rust-in-action-enum_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your enum/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
/// //```rust,compile_fail,ignore
#[derive(Debug)]
pub enum FileOpenMode {
  Read,
  Write,
  Append,
  Truncate,
}

#[derive(Debug)]
pub enum FileHandle {
  Handle(usize),
  None,
}

#[derive(Debug)]
pub enum FileState {
  PendingCreation,
  Created(FileOpenMode),
  Opened(FileOpenMode),
  Error(String),
  Closed,
  Deleted,
}

#[derive(Debug)]
pub struct File {
  name: String,
  data: Vec<u8>,
  state: FileState,
  handle: FileHandle,
}
// impl Display for FileState {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//      match *self {
//          FileState::Opened(x) => write!(f, "OPEN"),     // <4> Sneakily, we can make use of `write!` to do the grunt work for us. Strings already implement `Display` themselves, so there's very little left for us to do.
//          FileState::Closed => write!(f, "CLOSED"), // <4>
//      }
//    }
// }

// impl Display for File {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//       write!(f, "<{} ({})>", self.name, self.state)  // <5> We can rely on the FileState Display implementation in our own code
//    }
// }
impl File {
  pub fn new(name: &str) -> File {
    File {
      name: String::from(name),
      data: Vec::new(),
      state: FileState::PendingCreation, // <1>
      handle: FileHandle::None, // <1>
    }
  }

  pub fn from_options(name: &str, state: FileState, handle: FileHandle) -> File {
    File {
      name: String::from(name),
      data: Vec::new(),
      state: state,
      handle: handle,
    }
  }
}

fn main() {
  let f1 = File::new("f1.txt");
  let f2 = File::from_options("f2.txt",
                   FileState::Opened(FileOpenMode::Read),
                   FileHandle::Handle(123)
               );
  let f3 = File::from_options("f3.txt",
                   FileState::Opened(FileOpenMode::Write),
                   FileHandle::None
               );

  let mut files = [f1, f2, f3];

  for f in &files {
    println!("{:?}", f);
  }

  // uh oh, disk failure
  for ref mut f in &mut files {
    f.state = FileState::Error(String::from("disk read failure"));
    println!("{:?}", f);
  }
}

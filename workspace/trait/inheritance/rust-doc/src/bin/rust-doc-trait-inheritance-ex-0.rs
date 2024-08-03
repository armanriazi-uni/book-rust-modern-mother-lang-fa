#![allow(dead_code, unused_variables)]
use std::fmt;
/// rust-doc-trait-inheritance-ex-1
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-doc-trait-inheritance_bin --bin rust-doc-trait-inheritance-ex-0```
///
/// ```cargo doc  --package rust-doc-trait-inheritance_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-trait-inheritance_bin ```
///
/// ## What
/// `Supertrait-multi-trait`
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
///  `TODO`
///
  trait Person {
      fn name(&self) -> String;
  }
  trait Student: Person {
      fn university(&self) -> String;
  }
   
  trait Programmer {
      fn fav_language(&self) -> String;
  }
  trait CompSciStudent: Programmer + Student {
      fn git_username(&self) -> String;
  }
  fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
      format!(
     "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
          student.name(),
          student.university(),
          student.fav_language(),
          student.git_username()
      )
  }
  struct ComputerScienceStudent {
      name: String,
      university: String,
      fav_language: String,
      git_username: String,
  }
  impl Person for ComputerScienceStudent {
      fn name(&self) -> String {
          self.name.clone()
      }
  }
  impl Student for ComputerScienceStudent {
      fn university(&self) -> String {
          self.university.clone()
      }
  }
  impl Programmer for ComputerScienceStudent {
      fn fav_language(&self) -> String {
          self.fav_language.clone()
      }
  }
  impl CompSciStudent for ComputerScienceStudent {
      fn git_username(&self) -> String {
          self.git_username.clone()
      }
  }
  fn main() {
      let cs_student = ComputerScienceStudent {
          name: "John Doe".to_string(),
          university: "Example University".to_string(),
          fav_language: "Rust".to_string(),
          git_username: "johndoe123".to_string(),
      };
      println!("{}", comp_sci_student_greeting(&cs_student));
  }//
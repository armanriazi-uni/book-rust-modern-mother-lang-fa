#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-doc-trait-associate-type-and-trait_bin --bin rust-doc-trait-associate-type-and-trait-main```
///
/// ```cargo doc  --package rust-doc-trait-associate-type-and-trait_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-trait-associate-type-and-trait_bin ```
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
///  `TODO`
///
///

struct Counter {
  count: u32,
}

///sample of asscociate type impl
impl Counter {
  fn new() -> Counter {
      Counter { count: 0 }
  }
}

///sample of asscociate trait impl because of ret Option<Self::Item>
impl Iterator for Counter {
  type Item= u32;
  fn next(&mut self) -> Option<Self::Item> {
      // --snip--
      if self.count < 5 {
          self.count += 1;
          Some(self.count)
      } else {
          None
      }
  }
}

/* With generic type substitute associated type

pub trait Iterator<T> {
  fn next(&mut self) -> Option<T>;
}
*/
fn main() {

  let mut ct=Counter{count:3};
  println!("{:?}",&ct.next());
  println!("{:?}",&ct.next());
  println!("{:?}",&ct.next());
  println!("{:?}",&ct.next());
  println!("{:?}",&ct.next());

}

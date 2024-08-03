#![allow(dead_code, unused_variables)]

/// rust-doc-struct-bin-ex-8
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-struct_bin --bin rust-doc-struct-bin-ex-8```
///
/// ## What
///  `display`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `Value(0, 1, 12.078)`
///
/// ## Example
/// //```rust,compile_fail,ignore

#[derive(Debug)]
struct Value<V>(usize, usize, V);

struct SparseMatrix<V> {
    values: Vec<Value<V>>
}

// impl SparseMatrix {
//     fn
// }



fn main () {
  let val = Value(0, 1, 12.078);//SparseMatrix{values: Value{vec![0, 1, 12.078]}};
  println!("{:?}", val.values);
}

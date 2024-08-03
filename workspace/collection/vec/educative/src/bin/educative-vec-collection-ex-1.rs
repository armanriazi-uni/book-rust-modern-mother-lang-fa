#![allow(dead_code, unused_variables)]
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-vec-collection_bin --bin educative-vec-collection-ex-1```
///
/// ```cargo doc  --package educative-vec-collection_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-vec-collection_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// Then we shrink to fit, which does not give us any more certainty about how much space has been claimed on the heap. However,
/// the length of the vector is still 5. This example demonstrates that the length of a vector (the number of elements present) is different from the space the vector occupies on the heap.
///
/// # Arguments
///
/// * `tag` - This is the [str] to [find] the ['oo']
///
/// //# Example
///
fn main(){
  let mut vec = vec![1, 2, 3, 4, 5];
  assert_eq!(vec.len(), 5);
  vec.reserve(2);

  assert_eq!(vec.len(), 5);
  assert!(vec.capacity() >= 7 );

  vec.shrink_to_fit();
  assert_eq!(vec.len(), 5);
  assert!(vec.capacity() == 5 );
}

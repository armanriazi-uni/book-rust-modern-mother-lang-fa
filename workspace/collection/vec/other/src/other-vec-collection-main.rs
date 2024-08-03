#![allow(dead_code, unused_variables)]
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p other-vec-collection_bin --bin other-vec-collection-main```
///
/// ```cargo doc  --package other-vec-collection_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package other-vec-collection_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `tag` - This is the [str] to [find] the ['oo']
///
/// # Example

/// ## Result
///
/// ## Trace
/// [other-vec-collection-doc-main](../rust_in_action_vec_collection_trace1/index.html)

#[derive(Debug)]
struct Items{
   is_ordered:bool
}
fn main(){
   let it1= Items{is_ordered:true};
   let it2= Items{is_ordered:true};
   let items: Vec<Items>= vec![it1,it2];
  for mut item in items{
      item.is_ordered = true;
  }
}

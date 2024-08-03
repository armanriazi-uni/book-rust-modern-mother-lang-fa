#![allow(dead_code, unused_variables)]

use std::thread;

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-thread-rayon_bin --bin rust-doc-thread-rayon-main```
///
/// ```cargo doc  --package rust-doc-thread-rayon_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-thread-rayon_bin```
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

///
/// ```compile_fail,ignore

fn main(){
    let mut v = vec![5, 1, 8, 22, 0, 44];
    quick_sort(&mut v);
    assert_eq!(v, vec![0, 1, 5, 8, 22, 44]);
}
fn quick_sort<T:PartialOrd+Send>(v: &mut [T]) {
   if v.len() > 1 {
       let mid = partition(v);
       let (lo, hi) = v.split_at_mut(mid);
       rayon::join(|| quick_sort(lo),
                   || quick_sort(hi));
   }
}

// Partition rearranges all items `<=` to the pivot
// item (arbitrary selected to be the last item in the slice)
// to the first half of the slice. It then returns the
// "dividing point" where the pivot is placed.
fn partition<T:PartialOrd+Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}

#![allow(dead_code, unused_variables)]


/// master-rust-smartpointer-refcell-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p master-rust-smartpointer-ref-cell_bin --bin  master-rust-smartpointer-refcell-ex-2```
///
/// ```cargo doc  --package master-rust-smartpointer-ref-cell_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-smartpointer-ref-cell_bin ```
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
/// //```rust,no_run,compile_fail,ignore
use std::cell::Cell;
struct Point {
    x: u8,
    y: u8,
    cached_sum: Cell<Option<u8>>
}
impl Point {
    fn sum(&self) -> u8 {
        
        match self.cached_sum.get() {
            Some(sum) => {
                println!("Got from cache: {}", sum);
                sum
                },
            None => {
                let new_sum = self.x + self.y;
                self.cached_sum.set(Some(new_sum));
                println!("Set cache: {}", new_sum);
                new_sum
            }
        }
    }
}
fn main() {
    let p = Point { x: 8, y: 9, cached_sum: Cell::new(None) };
    println!("Summed result: {}", p.sum());
    println!("Summed result: {}", p.sum());
}

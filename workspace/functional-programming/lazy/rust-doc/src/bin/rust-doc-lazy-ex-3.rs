#![allow(dead_code, unused_variables)]
/// rust-doc-lazy-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-lazy_bin --bin rust-doc-lazy-ex-3```
///
/// ```cargo doc  --package rust-doc-lazy_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-lazy_bin ```
///
/// ## What
/// `GlobalState`
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

pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match &self.value {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
fn main() {
    let mut cacher = Cacher::new(|a| a + 2);

    println!("The value of 5 is {}", cacher.value(5));
    println!("The value of 6 is {}", cacher.value(6));
    println!("The value of 2 is {}", cacher.value(2));

    // Since the value for 5 has already been calculated and stored,
    // calling it again should return the cached value instead of recalculating.
    //println!("The value of 2 is {}", cacher.value(Option::None));
    println!("The value of 5 is {}", cacher.value(5));
}

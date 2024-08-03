#![allow(dead_code, unused_variables)]

/// lpxxn-creational-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p lpxxn-creational_bin --bin lpxxn-creational-ex-4```
///
/// ```cargo doc  --package lpxxn-creational_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package lpxxn-creational_bin```
///
/// ## What
///`Singeleton`
///
/// ## How
/// `TODO`
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///
/// //```rust,compile_fail,no_run,ignore
use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};
#[derive(Debug)]
struct Config {
    db_connection_str: String,
}
fn get_config() -> &'static Mutex<Config> {
    static mut CONF: MaybeUninit<Mutex<Config>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
        CONF.as_mut_ptr().write(Mutex::new(Config {
            db_connection_str: "test config".to_string(),
        }));
    });
    unsafe { &*CONF.as_ptr() }
}
fn main() {
    let f1 = get_config();
    println!("{:?}", f1);
    {
        let mut conf = f1.lock().unwrap();
        conf.db_connection_str = "hello".to_string();
    }
    let f2 = get_config();
    println!("{:?}", f2);
    let conf2 = f2.lock().unwrap();
    assert_eq!(conf2.db_connection_str, "hello".to_string())
}

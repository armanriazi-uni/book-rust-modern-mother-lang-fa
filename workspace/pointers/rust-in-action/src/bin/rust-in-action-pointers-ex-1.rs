#![allow(dead_code, unused_variables)]


/// rust-in-action-pointers-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-pointers-bin --bin  rust-in-action-pointers-ex-1```
///
/// ## What
/// `Memory Scan 1`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `GLOBAL:    0x56144a0ce000`
/// `local_str: 0x56144a0ce004`
/// `local_int: 0x7ffcd6a8fad4`
/// `boxed_int: 0x56144be11af0`
/// `boxed_str: 0x56144be11ad0`
/// `fn_int:    0x7ffcd6a8fa24`
///
/// ## Example
/// //``rust,no_run,compile_fail,ignore
static GLOBAL: i32 = 1000;             // <1>

fn noop() -> *const i32 {
    let noop_local = 12345;            // <2>
    &noop_local as *const i32          // <3>
}

fn main() {
    let local_str = "a";               // <4>
    let local_int = 123;               // <4>
    let boxed_str = Box::new('b');     // <4>
    let boxed_int = Box::new(789);     // <4>
    let fn_int = noop();               // <4>

    println!("GLOBAL:    {:p}", &GLOBAL as *const i32);
    println!("local_str: {:p}", local_str as *const str);
    println!("local_int: {:p}", &local_int as *const i32);
    println!("boxed_int: {:p}", Box::into_raw(boxed_int));
    println!("boxed_str: {:p}", Box::into_raw(boxed_str));
    println!("fn_int:    {:p}", fn_int);
}

#![allow(dead_code, unused_variables)]
use std::mem::size_of;

 static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
 static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

/// rust-in-action-pointers-ex-4
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-pointers-bin --bin  rust-in-action-pointers-ex-4```
///
/// ## What
/// `Memory Scan 3`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// ```
/// a (an unsigned integer):
///  location: 0x7ffde9af5660
///  size: 8 bytes
///  value: 42
///
/// b (a reference to B):
///  location: 0x7ffde9af5668
///  size: 8 bytes
///  points to: 0x55dfa8a10000
///
/// c (a "box" for C):
///  location: 0x7ffde9af5670
///  size: 16 bytes
///  points to: 0x55dfaa9fcad0
///
/// B (an array of 10 bytes):
///  location: 0x55dfa8a10000
///  size: 10 bytes
///  value: [99, 97, 114, 114, 121, 116, 111, 119, 101, 108]
///
/// C (an array of 11 bytes):
///  location: 0x55dfa8a1000a
///  size: 11 bytes
///  value: [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0]
/// ```
/// ## Example
/// ```
/// //Notice: it print 11 even without mention directly e.g  c::size_of::<Box<[u8]>>()
/// println!(" size: {:?} bytes", size_of::<Box<[u8]>>());
/// ```
/// related to: [rust-in-action-pointers-ex-5](rust-in-action-pointers-ex-5)
/// The Box<[u8]> type is a boxed byte slice. When we place values inside a box, ownership of the value moves to the owner of the box.
/// ```
/// let c: Box<[u8]> = Box::new(C);
/// ```
/// usize is the memory address size for the CPU the code is compiled for. That CPU is called the compile target
 fn main() {
 let a: usize = 42;

 let b: &[u8; 10] = &B;
 let c: Box<[u8]> = Box::new(C);

 println!("a (an unsigned integer):");
 println!(" location: {:p}", &a);
 println!(" size: {:?} bytes", size_of::<usize>());
 println!(" value: {:?}", a);
 println!();

 println!("b (a reference to B):");
 println!(" location: {:p}", &b);
 println!(" size: {:?} bytes", size_of::<&[u8; 10]>());
 println!(" points to: {:p}", b);
 println!();

 println!("c (a \"box\" for C):");
 println!(" location: {:p}", &c);
 println!(" size: {:?} bytes", size_of::<Box<[u8]>>()); //it print 11 even without mention directly e.g  c::size_of::<Box<[u8]>>()
 println!(" points to: {:p}", c);
 println!();

 println!("B (an array of 10 bytes):");
 println!(" location: {:p}", &B);
 println!(" size: {:?} bytes", size_of::<[u8; 11]>());
 println!(" value: {:?}", B);
 println!();

 println!("C (an array of 11 bytes):");
 println!(" location: {:p}", &C);
 println!(" size: {:?} bytes", size_of::<[u8; 11]>());
 println!(" value: {:?}", C);
 }

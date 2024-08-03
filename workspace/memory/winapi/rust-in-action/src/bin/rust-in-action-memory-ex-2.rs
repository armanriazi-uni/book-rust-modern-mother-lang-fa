#![allow(dead_code, unused_variables)]


/// rust-in-action-memory-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-memory-winapi_bin --bin  rust-in-action-memory-ex-2```
///
/// ## What
/// `Memory Scan 2`
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
/// //``rust,no_run,compile_fail,ignore
///
/// Wrong
// fn main() {
//     let mut n_nonzero = 0;
//     for i in 1..100 {    
//         let ptr = i as *const u8;
//         let byte_at_addr = unsafe { *ptr };

//         if byte_at_addr != 0 {
//             n_nonzero += 1;
//         }
//     }
//     println!("non-zero bytes in memory: {}", n_nonzero);
// }
// error: Segmentation fault
//Correct
 fn main() {
         let mut n_nonzero = 0;
        let data: Vec<u8> = (1..100).map(|i| i as u8).collect();
        for &byte in &data {
            if byte != 0 {
                n_nonzero += 1;
            }
        }
     
        println!("non-zero bytes in memory: {}", n_nonzero);
}
     //Output: non-zero bytes in memory: 99
   

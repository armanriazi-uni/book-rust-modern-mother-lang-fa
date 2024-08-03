#![allow(dead_code, unused_variables)]

///
/// rust-in-action-loop-flowcontrol-bin-ex-1
///
/// # Commands
///
/// ```cargo run -q -p rust-in-action-loop-flowcontrol_bin --bin rust-in-action-loop-flowcontrol-bin-ex-1```
///
/// ## What
/// `Testing how fast your computer can increment a counter`
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
///
/// ```rust,compile_fail,no_run
/// fn main() -> ! {
///   let j=1;
///   let i=100;
///     loop {
///         if i== j{
///           break;
///         }
///         else
///         {
///           j+=1;
///         }
///       };
// }
/// ```
///
 fn main() {
   let mut j=1;
   let i=100;
     loop {
         if &i== &j{
           break;
         }
         else
         {
           j+=1;
         }
       };
}

#![allow(dead_code, unused_variables)]

/// rust-in-action-dangling-bin-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-semantic_bin --bin  rust-in-action-semantic-bin-ex-3```
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

 //full ownership
 #[derive(Debug)]
 struct CubeSat {
   id: u64,
 }

 #[derive(Debug)]
 enum StatusMessage {
   Ok,
 }

 fn check_status(sat_id: CubeSat) -> CubeSat {
   println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
   sat_id
 }

 fn main () {
   let sat_a = CubeSat { id: 0 };
   let sat_b = CubeSat { id: 1 };
   let sat_c = CubeSat { id: 2 };

   let sat_a = check_status(sat_a);
   let sat_b = check_status(sat_b);
   let sat_c = check_status(sat_c);

   // "waiting" ...

   let sat_a = check_status(sat_a);
   let sat_b = check_status(sat_b);
   let sat_c = check_status(sat_c);
 }

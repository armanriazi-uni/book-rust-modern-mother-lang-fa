#![allow(dead_code, unused_variables)]

/// rust-in-action-dangling-bin-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-semantic_bin --bin  rust-in-action-semantic-bin-ex-2```
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

#[derive(Debug)]
 struct CubeSat {
   id: u64,
 }

 #[derive(Debug)]
 enum StatusMessage {
   Ok,
 }

 /*fn check_status(
   sat_id: CubeSat
 ) -> StatusMessage {
     //implicit code:  drop(sat_a)
   StatusMessage::Ok
 }*/

 fn check_status(sat_id: CubeSat) -> CubeSat {

    println!("{:?}: {:?}", sat_id,
                           StatusMessage::Ok);
    sat_id

  }

 fn main() {
   let sat_a = CubeSat { id: 0 };//we can see that sat_a starts its life with ownership over a CubeSat object:
   let sat_b = CubeSat { id: 1 };
   let sat_c = CubeSat { id: 2 };

   let a_status = check_status(sat_a);
   let b_status = check_status(sat_b);
   let c_status = check_status(sat_c);
   println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

   // "waiting" ...
  //  let a_status = check_status(sat_a);
  //  let b_status = check_status(sat_b);
  //  let c_status = check_status(sat_c); // Expose Error:  error[E0382]: use of moved value: `sat_c`
  //  println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
 }

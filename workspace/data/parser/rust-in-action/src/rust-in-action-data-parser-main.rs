#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-data-operator_bin --bin rust-in-action-data-parser-main```
///
/// ```cargo doc  --package rust-in-action-data-operator_bin--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-data-radix_bin```
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
/// 'nothing'
/// ## Example
/// //```rust,compile_fail,ignore
#[derive(Debug)] 
enum Event {
    Update,  
    Delete, 
    Unknown,
}
type Message = String; 

fn parse_log(line: &'static str) -> (Event, Message) { 
  let parts: Vec<&str> = line.splitn(2, ' ').collect(); 
  if parts.len() == 1 {
    return (Event::Unknown, String::from(line))
  }
  let event = parts[0];             
  let rest = String::from(parts[1]); 
  match event {
    "UPDATE" | "update" => (Event::Update, rest), // <8> When we match a known event, return structured data
    "DELETE" | "delete" => (Event::Delete, rest), // <8>
    _ => (Event::Unknown, String::from(line)), // <9> If we don't recognize the event type, return the whole line
  }
}
fn main() {
  let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

  for line in log.lines(){
    let parse_result = parse_log(line);
    println!("{:?}", parse_result);
  }
}

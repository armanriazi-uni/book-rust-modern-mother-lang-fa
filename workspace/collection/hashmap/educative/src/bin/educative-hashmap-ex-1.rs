#![allow(dead_code, unused_variables)]
use std::collections::HashMap;


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-hashmap_bin --bin educative-hashmap-ex-1```
///
/// ```cargo doc  --package educative-hashmap_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package educative-hashmap_bin```
///
/// ## What
/// `Insert and Update on HashMap`
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
/// //rust,compile_fail,no_run,ignore
//use std::collections::HashMap;

fn main(){
let text = r#"
Peter Piper picked a peck of pickled peppers
A peck of pickled peppers Peter Piper picked
If Peter Piper picked a peck of pickled peppers
Where is the peck of pickled peppers Peter Piper picked
"#.to_string().to_lowercase();

let mut word_count = HashMap::new();

for word in text.split_whitespace() {
    //let count = word_count.entry(word).or_insert(0);
    //*count += 1;
    //or
    word_count.entry(word).and_modify(|counter| *counter += 1).or_insert(0);
}

println!("{:#?}", word_count);
}

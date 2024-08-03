#![allow(dead_code, unused_variables)]
use std::collections::HashMap;


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p educative-hashmap_bin --bin educative-hashmap-main```
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
#[derive(PartialEq, Eq, Hash)]
struct Student<'a>{
    first_name: &'a str,
    last_name: &'a str,
}
#[derive(Debug)]
struct ScoreInfo<'a>{
    subject: &'a str,
    score: &'a str,
}
fn main() {
type Grades<'a> = HashMap<Student<'a>, ScoreInfo<'a>>;
let mut students: Grades = HashMap::new();
let jason = Student {
    first_name: "Jason",
    last_name: "Doe",
};
let grade1 = ScoreInfo {
    subject: "Math",
    score: "passing grade",
};
students.insert(jason, grade1);
println!("Jason's score info: {:?}",
students.get(
        &Student{first_name: "Jason", last_name: "Doe"}
).unwrap()
);
let mut student = HashMap::new();
student.insert("Robert".to_string(), 7);
student.insert("Jason".to_string(), 6);
student.insert("Marie".to_string(), 8);

for (k, v) in student.iter() {
    println!("{}'s score:\t{}", k, v);
}
for v in student.values() {
    println!("grade:\t {}", v);
}
// update HashMap
student.insert("Robert".to_string(), 7);
student.insert("Jason".to_string(), 6);
student.insert("Marie".to_string(), 8);
student.insert("Robert".to_string(), 5);
student.entry("Robert".to_string()).or_insert(88);
student.entry("Arman".to_string()).or_insert(100);

println!("Robert's grade:\t {}", student["Robert"]);
println!("Robert's grade:\t {}", student["Arman"]);
}

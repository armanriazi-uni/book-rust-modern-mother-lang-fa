#![allow(dead_code, unused_variables, unused_imports)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-egg-types-string_bin --bin rust-egg-types-string-main```
///
/// ```cargo egg  --package rust-egg-types-string_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --egg  --package rust-egg-types-string_bin```
///
/// ## What
/// Rust includes some tolerances to allow comparisons between stringing-point values.
/// These tolerances are defined as f32::EPSILON and f64::EPSILON. To be more precise,
/// it’s possible to get closer to how Rust is behaving under the hood, as the following small example shows.
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `blue
/// ab
/// blue rt
/// red
/// red
/// red
/// red
/// Red Tuesday
/// Histation
/// blue
/// red
/// hi
/// rust is fun!
/// nice weather
/// Interpolation Station
/// a
/// hello there
/// Happy Tuesday!
/// my shift key is sticky
/// `
///
/// ## Example
/// ```rust,compile_fail,ignore


fn str_slice(arg: &str){
    println!("{}",arg);
}
fn mystr(arg : String){
    println!("{}",arg);
}

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main(){

    str_slice("blue");
    str_slice(&String::from("abc")[0..2]);
    str_slice(" blue rt".trim());
    str_slice("red".into());

    mystr("red".to_string());
    mystr("red".to_owned());
    mystr("Red".to_lowercase());
    mystr("Red Monday".to_string().replace("Mon","Tues"));
    mystr(format!("Hi{}","station"));


    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());

}

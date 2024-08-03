#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]


/// Main
///
/// ## Commands
///
/// ```cargo run -q -p packtpub-types-string_bin --bin packtpub-types-string-main```
///
/// ```cargo egg  --package packtpub-types-string_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --egg  --package packtpub-types-string_bin```
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


fn main() {

    let s = String::from("Hello 中国");

    println!("S byte len = {}",s.len());

    println!("Num L's = {}",count_l(&s));

    for c in s.chars(){
        println!("{}",c);
    }

    for c in s.bytes(){
        println!("{}",c);
    }

    for (i,c) in s.char_indices(){
        println!("{} = {}",i,c);
    }
}

fn count_l(s:&str)->i32{
    let mut res = 0;
    for c in s.chars(){
        if c == 'l' {
            res +=1;
        }
    }
    res
}

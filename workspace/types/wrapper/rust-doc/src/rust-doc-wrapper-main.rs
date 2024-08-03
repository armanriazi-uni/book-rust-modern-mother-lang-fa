#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-wrapper_bin --bin rust-doc-wrapper-main```
///
/// ```cargo doc  --package rust-doc-wrapper_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-wrapper_bin```
///
/// ## What
// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your wrapper/func name]
///
/// # Return
/// `assert:true`
///
/// ## Example
/// ```rust,compile_fail,ignore
/// //error[E0369]: cannot add `T` to `T`
/// pub fn add<T>(a: T, b: T) -> T {
///    a + b
/// }

///```
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///     use proptest::prelude::*;
///     proptest! {
///         #[test]
///         fn test_add(a: i64, b: i64) {
///             assert_eq!(add(a, b), a+b);
///         }
///     }
/// }
///```
///

#[derive(Debug)]
 pub struct User{
    name:String,
    age:i32,
    height:i32,
    shoesize:i32,
}

impl User{
   pub fn simple_string(&self)->String{
        format!("{} - {} - {}cm - shoe:{}",self.name,self.age,self.height,self.shoesize)
    }

    pub fn grow(&mut self,h:i32){
        self.height += h;
    }

    pub fn die(self){
        println!("Dead {} ", self.simple_string());
    }
}


fn main() {

    let mut u = User{
        name:"Matt".to_string(),
        age:33,
        height:250,
        shoesize:10,
    };

    println!("User is {}",u.simple_string());
    u.grow(20);
    println!("User is {}",u.simple_string());
    u.die();


}

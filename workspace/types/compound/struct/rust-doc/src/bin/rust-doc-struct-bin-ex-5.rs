#![allow(dead_code, unused_variables)]

/// rust-doc-struct-bin-ex-5
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-struct_bin --bin rust-doc-struct-bin-ex-5``
///
/// ## What
///  `display`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `error[E0106]: missing lifetime specifier`
///
/// ## Example
/// ```rust,compile_fail,ignore
///struct User {
///    active: bool,
///    username: &str,
///    email: &str,
///    sign_in_count: String,
///}

///fn main() {
///    let user1 = User {
///        email: "someone@example.com",
///        username: "someusername123",
///        active: true,
///        sign_in_count: 1,
///    };
///}

fn main(){
    unimplemented!()
}

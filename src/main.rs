#![crate_name = "rust_all_in_one"]
#![allow(dead_code, unused_variables)]

#![doc(html_logo_url = "https://armanriazi-uni.github.io/site/assets/attachments/me.png")]


/// Main
///
/// # Commands
///
/// ```cargo run -q -p book-rust-modern-mother-lang-to-fa```
///
/// ```cargo doc  --workspace --message-format short --no-deps --open --color always```
///
/// ```cargo check --workspace --message-format=short --all-targets```
///
/// ```cargo test --doc  --workspace```
///
/// ```cargo clippy  --workspace --message-format=short --all-targets```
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
/// `nothing`
///
/// ## Example
/// ```rust,compile_fail,ignore
/// #[allow(dead_code)]
/// fn m(){
///     core::panicking::panic!("in crash");
///     unimplemented!()
/// }
/// ```



#[cfg(panic = "unwind")]
#[cfg(target_family = "unix")]
fn get_platform() -> String {
    "UNIX".into()
}


#[cfg(target_family = "windows")]
fn get_platform() -> String {
    "Windows".into()
}

fn greet_world() {
    
    println!("Hello, world!");
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
/// We have two get_platform and selected by conditional_features.
/// > Line 11 :(commented)
/// /// error: unneeded unit return type ^^^^ help: remove the `-> ()`
/// fn nothing()->(){
/// }

fn finish() -> impl std::process::Termination {
    let machine_kind = if cfg!(unix) {
        println!("I was running on a unix machine!");
        std::process::ExitCode::SUCCESS
    } else if cfg!(windows) {
        println!("I was running on a windows machine!");
        std::process::ExitCode::SUCCESS
    } else {
        println!("I was running on a unknown machine!");
        std::process::ExitCode::FAILURE
    };
}


fn main(){
     
unimplemented!();

}


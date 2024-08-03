#![allow(dead_code, unused_variables)]



/// rust-doc-std-path-ex-2
///
/// ## Commands
///
/// ```cargo run -q -p rust_doc_std_bin  --bin rust-doc-std-path-ex-2```
///
/// ```cargo doc  --package rust_doc_std_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust_doc_std_bin ```
///
/// ## What
/// `PATH`
///
/// ## How
/// The Path struct represents file paths in the underlying filesystem. There are two flavors of Path: posix::Path, for UNIX-like systems, and windows::Path, for Windows. The prelude exports the appropriate platform-specific Path variant.
/// A Path can be created from an OsStr, and provides several methods to get information from the file/directory the path points to.
/// A Path is immutable. The owned version of Path is PathBuf. The relation between Path and PathBuf is similar to that of str and String: a PathBuf can be mutated in-place, and can be dereferenced to a Path.
/// **Note that a Path is not internally represented as an UTF-8 string, but instead is stored as an OsString**. Therefore, converting a Path to a &str is not free and may fail (an Option is returned). However, a Path can be freely converted to an OsString or &OsStr using into_os_string and as_os_str, respectively.
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `new path is ./a/b/c/package.tgz`
///
/// ## Example
/// In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
///
/// //```compile_fail,ignore
use std::path::Path;
fn main() {
    let path = Path::new(".");
    let _display = path.display();
    let mut new_path = path.join("a").join("b");
    new_path.push("c");
    new_path.push("myfile.tar.gz");
    new_path.set_file_name("package.tgz");

    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}

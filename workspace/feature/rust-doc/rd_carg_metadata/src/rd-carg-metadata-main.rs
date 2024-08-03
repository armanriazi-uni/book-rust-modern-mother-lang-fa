#![allow(dead_code, unused_variables)]
/// Main
///
/// ## Commands
///
/// ```cargo build -q -p rd_carg_metadata_bin --bin rd-carg-metadata-main --message-format=json-render-diagnostics```
/// 
/// ```cargo run -q -p rd_carg_metadata_bin --bin rd-carg-metadata-main```
///
/// ```cargo doc  --package rd_carg_metadata_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rd-carg-metadata_bin```
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
/// ```
use std::process::{Stdio, Command};
use cargo_metadata::Message;

fn main(){
let mut command = Command::new("cargo")
    .args(&["build", "--message-format=json-render-diagnostics"])
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

let reader = std::io::BufReader::new(command.stdout.take().unwrap());
for message in cargo_metadata::Message::parse_stream(reader) {
    match message.unwrap() {
        // Message::CompilerMessage(msg) => {
        //     println!("{:?}", msg);
        // },
        // Message::CompilerArtifact(artifact) => {
        //     println!("{:?}", artifact);
        // },
        // Message::BuildScriptExecuted(script) => {
        //     println!("{:?}", script);
        // },
        Message::BuildFinished(finished) => {
            println!("{:?}", finished);
        },
        _ => () // Unknown message
    }
}

let output = command.wait().expect("Couldn't get cargo's exit status");
}
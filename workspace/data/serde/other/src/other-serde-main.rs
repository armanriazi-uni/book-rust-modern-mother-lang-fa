/// Main
///
/// ## Commands
///
/// ```cargo run -q -p other-serde_bin --bin other-serde-main```
///
/// ```cargo doc  --package other-serde_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package other-serde_bin ```
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
///  `TODO`
///
///
use std::io::{self, Read, Write};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Debug)]
struct Incoming {
    first: String,
    last: String,
}

#[derive(Serialize, Debug, Clone)]
struct Outgoing {
    name: String,
}

// Modified function signature to return Result
fn stream_things<R: Read, W: Write>(reader: R, mut writer: W) -> Result<(), Box<dyn std::error::Error>> {
    let incoming: Vec<Incoming> = serde_json::from_reader(reader)?;

    for a in incoming {
        let b = Outgoing {
            name: format!("{} {}", a.first, a.last),
        };
        serde_json::to_writer(&mut writer, &b)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // // Wrap stdin and stdout in BufReader and BufWriter for better performance
    // let reader = io::BufReader::new(io::stdin());
    // let writer = io::BufWriter::new(io::stdout());

    // // Call stream_things and handle any potential errors
    // match stream_things(reader, writer) {
    //     Ok(_) => println!("Stream completed successfully."),
    //     Err(e) => eprintln!("An error occurred: {}", e),
    // }

    // Ok(())
    unimplemented!()
}



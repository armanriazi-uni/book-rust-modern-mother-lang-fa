#![allow(dead_code, unused_variables)]
// use error_chain::error_chain;
// use serde::Deserialize;
// use std::fmt;


/// rust-doc-error-handling-ex-6
///
/// ## Commands
///
/// ```cargo run -q -p rust-doc-error-handling_bin --bin  rust-doc-error-handling-ex-6```
///
/// ## What
/// `Propagating errors`
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
/// //``rust,no_run,compile_fail,ignore



// error_chain! {
//     foreign_links {
//         Reader(csv::Error);
//     }
// }

// #[derive(Debug, Deserialize)]
// struct Rgb {
//     red: u8,
//     blue: u8,
//     green: u8,
// }

// impl Rgb {
//     fn from_reader(csv_data: &[u8]) -> Result<Rgb> {
//         let color: Rgb = csv::Reader::from_reader(csv_data)
//             .deserialize()
//             .nth(0)
//             .ok_or("Cannot deserialize the first CSV record")?
//             .chain_err(|| "Cannot deserialize RGB color")?;

//         Ok(color)
//     }
// }

// impl fmt::UpperHex for Rgb {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let hexa = u32::from(self.red) << 16 | u32::from(self.blue) << 8 | u32::from(self.green);
//         write!(f, "{:X}", hexa)
//     }
// }

// fn run() -> Result<()> {
//     let csv = "red,blue,green,102,256,204";

//     let rgb = Rgb::from_reader(csv.as_bytes()).chain_err(|| "Cannot read CSV data")?;
//     println!("{:?} to hexadecimal #{:X}", rgb, rgb);

//     Ok(())
// }

fn main() {
    unimplemented!();
    // if let Err(ref errors) = run() {
    //     eprintln!("Error level - description");
    //     errors
    //         .iter()
    //         .enumerate()
    //         .for_each(|(index, error)| eprintln!("└> {} - {}", index, error));

    //     if let Some(backtrace) = errors.backtrace() {
    //         eprintln!("{:?}", backtrace);
    //     }

    //     // In a real use case, errors should handled. For example:
    //     // ::std::process::exit(1);
    // }
}

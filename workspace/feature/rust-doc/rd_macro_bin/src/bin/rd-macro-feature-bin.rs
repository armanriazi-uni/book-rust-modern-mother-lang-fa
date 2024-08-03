#![allow(dead_code, unused_variables)]
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rd-macro-feature_bin --bin rd-macro-feature-bin```
///
/// ```cargo doc  --package rd-macro-feature_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rd-macro-feature_bin```
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

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate features;

features! {
    pub mod ux {
        const JsonOutput = 0b10000000,
        const VerboseOutput = 0b01000000
    }
}

features! {
    pub mod srv {
        const Http2Downloading = 0b10000000,
        const BitTorrentDownloading = 0b01000000
    }
}

fn main() {
    // Parse CLI args, environment, read config file etc...
    srv::enable(srv::BitTorrentDownloading);
    ux::enable(ux::VerboseOutput);

    if srv::is_enabled(srv::Http2Downloading) {
        println!("Downloading via http2...");
    } else if srv::is_enabled(srv::BitTorrentDownloading) {
        println!("Downloading via bit torrent...");
    } else {
        println!("Downloading the old fashioned way...");
    }

    if ux::is_enabled(ux::VerboseOutput) {
        println!("COOL");
    }
}
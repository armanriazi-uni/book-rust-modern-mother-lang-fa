#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ````RUST_BACKTRACE=full cargo run -q -p master-rust-trait-associate-type_bin --bin master-rust-trait-associate-type-main```
///
/// ```cargo doc  --package master-rust-trait-associate-type_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master-rust-trait-associate-type_bin ```
///
/// ## What
/// `TODO`
///
/// ## How
/// > two kinds of methods within a trait:
/// > self is just a type alias to Self,which refers to the type on which the trait is being implemented
/// > Sample Associated method: pause, Instance methods: play
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



pub trait Playable {
  fn play(&self);
  fn pause() {
      println!("Paused");
  }
}


struct Audio(String);
struct Video(String);

impl Playable for Audio {
    fn play(&self) {
        println!("🎵 Now playing: {}", self.0);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("🎵 Now playing: {}", self.0);
    }
}

fn main() {
    println!("Super player!");
    let audio = Audio("ambient_music.mp3".to_string());
    let video = Video("big_buck_bunny.mkv".to_string());
    audio.play();
    video.play();
}

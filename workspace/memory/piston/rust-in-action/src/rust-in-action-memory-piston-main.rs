
#![allow(dead_code, unused_variables)]
/// provides mathematical operations and conversion functionality for 2D vectors.
use graphics::math::{Vec2d, add, mul_scalar};
/// piston_window provides the tools to create a GUI program and draws shapes to it.
use piston_window::*;

use rand::prelude::*;                            // <3>
/// std::alloc provides facilities for controlling memory allocation.
use std::alloc::{GlobalAlloc, System, Layout};

use std::time::Instant;                          // <5>



/// Main
///
/// ## Commands
/// Run in in quiet mode
/// ```cargo run -q 2> alloc.tsv -p rust-in-action-memory-piston_bin --bin rust-in-action-memory-piston-main```
/// Views the first 10 lines of output
/// ``` head alloc.tsv```
/// `alloc.plot` is a script used to generate plot gui with gnuplot
///
/// ```cargo doc  --package rust-in-action-memory-piston_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-memory-piston_bin ```
///
/// ## What
/// `Dynamic memory allocation ,Piston Memory`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `Two col of random numbers`
///
/// ## Example
///  `TODO`
///
/// //```rust,compile_fail,no_run,ignore
///

/// marks the following value (ALLOCATOR) as satisfying the GlobalAlloc trait.
#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

/// Prints the time taken for each allocation to STDOUT as the program runs. This provides a fairly accurate indication of the time taken for dynamic memory allocation.
struct ReportingAllocator;

/// 'System.alloc(layout);' Defers the actual memory allocation to the system’s default memory allocator
unsafe impl GlobalAlloc for ReportingAllocator {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    let start = Instant::now();
    let ptr = System.alloc(layout);              // <8>
    let end = Instant::now();
    let time_taken = end - start;
    let bytes_requested = layout.size();

    eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());
    ptr
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    System.dealloc(ptr, layout);
  }
}

/// Contains the data that is useful for the lifetime of the program
struct World {                      // <9>
  current_turn: u64,                // <9>
  particles: Vec<Box<Particle>>,    // <9>
  height: f64,                      // <9>
  width: f64,                       // <9>
  rng: ThreadRng,                   // <9>
}

/// Defines an object in 2D space
struct Particle {                   // <10>
  height: f64,                      // <10>
  width: f64,                       // <10>
  position: Vec2d<f64>,             // <10>
  velocity: Vec2d<f64>,             // <10>
  acceleration: Vec2d<f64>,         // <10>
  color: [f32; 4],                  // <10>
}

/// Starts at a random position along the bottom of the window
/// Rises vertically over time
/// Increases the speed of the rise over time
impl Particle {
  fn new(world : &World) -> Particle {
    let mut rng = thread_rng();
    let x = rng.gen_range(0.0..=world.width);      // <11>
    let y = world.height;                          // <11>
    let x_velocity = 0.0;                          // <12>
    let y_velocity = rng.gen_range(-2.0..0.0);     // <12>
    let x_acceleration = 0.0;                      // <13>
    let y_acceleration = rng.gen_range(0.0..0.15); // <13>

    Particle {
      height: 4.0,
      width: 4.0,
      position: [x, y].into(),                     // into() converts the arrays of type [f64; 2]into Vec2d
      velocity: [x_velocity, y_velocity].into(),   // <14>
      acceleration: [x_acceleration,
	                 y_acceleration].into(),       // <14>
      color: [1.0, 1.0, 1.0, 0.99],               // Inserts a fully saturated white that has a tiny amount of transparency
    }
  }

  /// > Moves the particle to its next position
  /// > Slows down the particle’s rate of increase as it travels across the screen
  /// > Makes the particle more transparent over time
  fn update(&mut self) {
    self.velocity = add(self.velocity,
	                    self.acceleration);        // <16>
    self.position = add(self.position,
	                    self.velocity);            // <16>
    self.acceleration = mul_scalar(                // <17>
      self.acceleration,                           // <17>
      0.7                                          // <17>
    );                                             // <17>
    self.color[3] *= 0.995;
  }
}

impl World {
  /// > Uses Box<Particle> rather than Particle to incur an extra memory allocation when every particle is created
  fn new(width: f64, height: f64) -> World {
    World {
      current_turn: 0,
      particles: Vec::<Box<Particle>>::new(),      // <19>
      height: height,
      width: width,
      rng: thread_rng(),
    }
  }

  /// > Creates a Particle as a local variable on the stack
  /// > Takes ownership of particle, moving its data to the heap, and creates a reference to that data on the stack
  /// Pushes the reference into self.shapes
  ///
  fn add_shapes(&mut self, n: i32) {
    for _ in 0..n.abs() {
      let particle = Particle::new(&self);         // <20>
      let boxed_particle = Box::new(particle);     // <21>
      self.particles.push(boxed_particle);         // <22>
    }
  }

  fn remove_shapes(&mut self, n: i32) {
    for _ in 0..n.abs() {
      let mut to_delete = None;

      let particle_iter = self.particles           // particle_iter is split into its own variable to more easily fit on the page.
        .iter()                                    // <23>
        .enumerate();                              // <23>

      // For n iterations, removes the first particle that’s invisible. If there are no invisible particles, then removes the oldest.
      // [TODO!] clippy error this loop never actually loops
      // for (i, particle) in particle_iter {         // <24>
      //   if particle.color[3] < 0.02 {              // <24>
      //     to_delete = Some(i);                     // <24>
      //   }                                          // <24>
      //   break;                                     // <24>
      // }                                            // <24>
                                                   // <24>
      if let Some(i) = to_delete {                 // <24>
        self.particles.remove(i);                  // <24>
      } else {                                     // <24>
        self.particles.remove(0);                  // <24>
      };                                           // <24>
    }
  }

  fn update(&mut self) {
    let n = self.rng.gen_range(-3..=3);            // <25>

    if n > 0 {
      self.add_shapes(n);
    } else {
      self.remove_shapes(n);
    }

    self.particles.shrink_to_fit();
    for shape in &mut self.particles {
      shape.update();
    }
    self.current_turn += 1;
  }
}

fn main() {
  let (width, height) = (1280.0, 960.0);
  let mut window: PistonWindow = WindowSettings::new(
    "particles", [width, height]
  )
  .exit_on_esc(true)
  .build()
  .expect("Could not create a window.");

  let mut world = World::new(width, height);
  world.add_shapes(1000);

  while let Some(event) = window.next() {
    world.update();

    window.draw_2d(&event, |ctx, renderer, _device| {
      clear([0.15, 0.17, 0.17, 0.9], renderer);

      for s in &mut world.particles {
        let size = [s.position[0], s.position[1], s.width, s.height];
        rectangle(s.color, size, ctx.transform, renderer);
      }
    });
  }
}

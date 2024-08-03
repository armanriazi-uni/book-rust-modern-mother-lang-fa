
///
/// ## Example
/// ```
/// fn main() {
///
///   // PARAMETERS
///
///   let context_lines = 2; //show 2 line before and after of finding line with 'needle' line
///
///   let needle = "oo";
///
///   let haystack = "0000,
///
/// bedroom 1111
///
/// 2222
///
/// 3333
///
/// 4444 books.
///
/// 5555
///
/// 6666
///
/// 7777
///
/// 8888
///
/// noon 9999";
///
///
///
///   // INITIALIZATION
///
///   let mut tags: Vec<usize> = Vec::new(); // <1> `tags` holds line numbers where matches occur
///
///   let mut ctx: Vec<Vec<(usize, String)>> = Vec::new(); // <2> `ctx` contains a vector per match to hold that match's context lines
///
///
///
///   // PASS 1
///
///   for (i, line) in haystack.lines().enumerate() { // <3> iterate through the lines, recording line numbers where matches are encountered
///
///     if line.contains(needle) {
///
///       tags.push(i);
///
///
///
///       let v = Vec::with_capacity(2*context_lines + 1); // <4> <5> `Vec::with_capacity(_n_)` reserves space for _n_ items
///
///       ctx.push(v);
///
///     }
///
///   }
///
///   if cfg!(debug_assertions) {
///
///
///
///                               eprintln!("--------------context_lines:{:?}--------------",context_lines);
///
///                               eprintln!("--------------find:{:?}--------------",needle);
///
///                               eprintln!("--------------tags:{:?}--------------",tags);
///
///   }
///
///
///
///   if tags.len() == 0 { // <6> When there are no matches, exit early
///
///     return;
///
///   }
///
///
///
///   // PASS 2
///
///   for (i, line) in haystack.lines().enumerate() { // <7> For each tag, at every line, check to see if we are nearby a match. When we are, add that line to the relevant `Vec<T>` within `ctx`.
///
///
///
///                      if cfg!(debug_assertions) {
///
///                               eprintln!("--------------i:{:?}--------------",i)
///
///                             }
///
///
///
///     for (j, tag) in tags.iter().enumerate() {
///
///       let lower_bound = tag.saturating_sub(context_lines); // <8> `usize.saturating_sub()` returns 0, rather than underflowing
///
///       let upper_bound = tag + context_lines;
///
///
///
///                     if cfg!(debug_assertions) {
///
///                         eprintln!("state--------------> j: {:?},lower_bound:{:?},upper_bound:{:?}--------------\n",j,lower_bound,upper_bound);
///
///
///
///                       }
///
///
///
///         if (i >= lower_bound) && (i <= upper_bound) {
///
///             let line_as_string = String::from(line); // <9> Copy `line` into a new `String` and store that locally for each match
///
///             let local_ctx = (i, line_as_string);
///
///
///
///                       if cfg!(debug_assertions) {
///
///                         eprintln!("local bounded: {:?}", local_ctx);
///
///
///
///                       }
///
///
///
///             ctx[j].push(local_ctx);
///
///              if cfg!(debug_assertions) {
///
///
///
///                         eprintln!("bounded-update-ctx[{:?}]: {:?}\n",j,ctx[j])
///
///                       }
///
///         }else {
///
///                       if cfg!(debug_assertions) {
///
///                         eprintln!("outer bound\n")
///
///                       }
///
///         }
///
///     }
///
///   }
///
///   eprintln!("---------------------output--------------");
///
///   // OUTPUT
///
///   for local_ctx in ctx.iter() {
///
///
///
///      eprintln!("local_ctx: {:?}", local_ctx);
///
///
///
///     for &(i, ref line) in local_ctx.iter() { // <10> `ref line` informs the compiler that we wish to borrow this value, rather than move it. These two terms are explained fully later in later chapters.
///
///       let line_num = i ;//+ 1;
///
///       println!("{}: {}", line_num, line);
///
///     }
///
///   }
///
/// }
///
/// ```
///
/// ## Result
///
/// ```
/// --------------context_lines:2--------------
///
/// --------------find:"oo"--------------
///
/// --------------tags:[1, 4, 9]--------------
///
/// --------------i:0--------------

/// state--------------> j: 0,lower_bound:0,upper_bound:3--------------
///
/// local bounded: (0, "0000,")
///
/// bounded-update-ctx[0]: [(0, "0000,")]
///
/// state--------------> j: 1,lower_bound:2,upper_bound:6--------------
///
/// outer bound
///
/// state--------------> j: 2,lower_bound:7,upper_bound:11--------------
///
/// outer bound
///
/// --------------i:1--------------
/// state--------------> j: 0,lower_bound:0,upper_bound:3--------------
///
/// local bounded: (1, "bedroom 1111")
///
/// bounded-update-ctx[0]: [(0, "0000,"), (1, "bedroom 1111")]
///
/// state--------------> j: 1,lower_bound:2,upper_bound:6--------------
///
/// outer bound
///
/// state--------------> j: 2,lower_bound:7,upper_bound:11--------------
///
/// outer bound
///
/// --------------i:2--------------
/// state--------------> j: 0,lower_bound:0,upper_bound:3--------------
///
/// local bounded: (2, "2222")
///
/// bounded-update-ctx[0]: [(0, "0000,"), (1, "bedroom 1111"), (2, "2222")]
///
/// state--------------> j: 1,lower_bound:2,upper_bound:6--------------
///
/// local bounded: (2, "2222")
///
/// bounded-update-ctx[1]: [(2, "2222")]
///
/// state--------------> j: 2,lower_bound:7,upper_bound:11--------------
///
/// outer bound
///
/// --------------i:3--------------
/// state--------------> j: 0,lower_bound:0,upper_bound:3--------------
///
/// local bounded: (3, "3333")
///
/// bounded-update-ctx[0]: [(0, "0000,"), (1, "bedroom 1111"), (2, "2222"), (3, "3333")]
///
/// state--------------> j: 1,lower_bound:2,upper_bound:6--------------
///
/// local bounded: (3, "3333")
///
/// bounded-update-ctx[1]: [(2, "2222"), (3, "3333")]
///
/// state--------------> j: 2,lower_bound:7,upper_bound:11--------------
///
/// outer bound
///
/// --------------i:4--------------
/// state--------------> j: 0,lower_bound:0,upper_bound:3--------------
///
/// outer bound
///
/// state--------------> j: 1,lower_bound:2,upper_bound:6--------------
///
/// local bounded: (4, "4444 books.")
///
/// bounded-update-ctx[1]: [(2, "2222"), (3, "3333"), (4, "4444 books.")]
///
/// state--------------> j: 2,lower_bound:7,upper_bound:11--------------
///
/// outer bound
///
/// --------------i:5--------------
/// state--------------> j: 0,lower_bound:0,upper_bound:3--------------
///
/// outer bound
///
/// state--------------> j: 1,lower_bound:2,upper_bound:6--------------
///
/// local bounded: (5, "5555")
///
/// bounded-update-ctx[1]: [(2, "2222"), (3, "3333"), (4, "4444 books."), (5, "5555")]
///
/// state--------------> j: 2,lower_bound:7,upper_bound:11--------------
///
/// outer bound
///
/// --------------i:6--------------
/// state--------------> j: 0,lower_bound:0,upper_bound:3--------------
///
/// outer bound
///
/// state--------------> j: 1,lower_bound:2,upper_bound:6--------------
///
/// local bounded: (6, "6666")
///
/// bounded-update-ctx[1]: [(2, "2222"), (3, "3333"), (4, "4444 books."), (5, "5555"), (6, "6666")]
///
/// state--------------> j: 2,lower_bound:7,upper_bound:11--------------
///
/// outer bound
///
/// --------------i:7--------------
/// state--------------> j: 0,lower_bound:0,upper_bound:3--------------
///
/// outer bound
///
/// state--------------> j: 1,lower_bound:2,upper_bound:6--------------
///
/// outer bound
///
/// state--------------> j: 2,lower_bound:7,upper_bound:11--------------
///
/// local bounded: (7, "7777")
///
/// bounded-update-ctx[2]: [(7, "7777")]
///
/// --------------i:8--------------
/// state--------------> j: 0,lower_bound:0,upper_bound:3--------------
///
/// outer bound
///
/// state--------------> j: 1,lower_bound:2,upper_bound:6--------------
///
/// outer bound
///
/// state--------------> j: 2,lower_bound:7,upper_bound:11--------------
///
/// local bounded: (8, "8888")
///
/// bounded-update-ctx[2]: [(7, "7777"), (8, "8888")]
///
/// --------------i:9--------------
/// state--------------> j: 0,lower_bound:0,upper_bound:3--------------
///
/// outer bound
///
/// state--------------> j: 1,lower_bound:2,upper_bound:6--------------
///
/// outer bound
///
/// state--------------> j: 2,lower_bound:7,upper_bound:11--------------
///
/// local bounded: (9, "noon 9999")
///
/// bounded-update-ctx[2]: [(7, "7777"), (8, "8888"), (9, "noon 9999")]
///
/// ---------------------output--------------
/// local_ctx: [(0, "0000,"), (1, "bedroom 1111"), (2, "2222"), (3, "3333")]
///
/// 0: 0000,
///
/// 1: bedroom 1111
///
/// 2: 2222
///
/// 3: 3333
///
/// local_ctx: [(2, "2222"), (3, "3333"), (4, "4444 books."), (5, "5555"), (6, "6666")]
///
/// 2: 2222
///
/// 3: 3333
///
/// 4: 4444 books.
///
/// 5: 5555
///
/// 6: 6666
///
/// local_ctx: [(7, "7777"), (8, "8888"), (9, "noon 9999")]
///
/// 7: 7777
///
/// 8: 8888
///
/// 9: noon 9999
///
/// ```

fn main(){
    unimplemented!()
}

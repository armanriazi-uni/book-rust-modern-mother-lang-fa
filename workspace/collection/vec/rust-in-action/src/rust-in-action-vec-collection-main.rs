#![allow(dead_code, unused_variables)]
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p rust-in-action-vec-collection_bin --bin rust-in-action-vec-collection-main```
///
/// ```cargo doc  --package rust-in-action-vec-collection_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-in-action-vec-collection_bin```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `tag` - This is the [str] to [find] the ['oo']
///
/// # Example
/// `1: Every face, every shop,`
///
/// `2: bedroom window, public-house, and`

/// `3: dark square is a picture`
///
/// `4: feverishly turned--in search of what?`
///
/// `3: dark square is a picture`
///
/// `4: feverishly turned--in search of what?`
///
/// `5: It is the same with books.`
///
/// `6: What do we seek`
///
/// `7: through millions of pages?`
///
///
/// ## Result
///
/// ## Trace
/// [rust-in-action-vec-collection-doc-main](../rust_in_action_vec_collection_trace1/index.html)

fn main() {
  // PARAMETERS
  let context_lines = 2; //show 2 line before and after of finding line with 'needle' line
  let needle = "oo";
  let haystack = "1: Every face, every shop,
2: bedroom window, public-house, and
3: dark square is a picture
4: feverishly turned--in search of what?
3: dark square is a picture
4: feverishly turned--in search of what?
5: It is the same with books.
6: What do we seek
7: through millions of pages?";

  // INITIALIZATION
  let mut tags: Vec<usize> = Vec::new(); // <1> `tags` holds line numbers where matches occur
  let mut ctx: Vec<Vec<(usize, String)>> = Vec::new(); // <2> `ctx` contains a vector per match to hold that match's context lines

  // PASS 1
  for (i, line) in haystack.lines().enumerate() { // <3> iterate through the lines, recording line numbers where matches are encountered
    if line.contains(needle) {
      tags.push(i);

      let v = Vec::with_capacity(2*context_lines + 1); // <4> <5> `Vec::with_capacity(_n_)` reserves space for _n_ items
      ctx.push(v);
    }
  }


  if tags.len() == 0 { // <6> When there are no matches, exit early
    return;
  }

  // PASS 2
  for (i, line) in haystack.lines().enumerate() { // <7> For each tag, at every line, check to see if we are nearby a match. When we are, add that line to the relevant `Vec<T>` within `ctx`.


    for (j, tag) in tags.iter().enumerate() {
      let lower_bound = tag.saturating_sub(context_lines); // <8> `usize.saturating_sub()` returns 0, rather than underflowing
      let upper_bound = tag + context_lines;

        if (i >= lower_bound) && (i <= upper_bound) {
            let line_as_string = String::from(line); // <9> Copy `line` into a new `String` and store that locally for each match
            let local_ctx = (i, line_as_string);

            ctx[j].push(local_ctx);

        }
    }
  }

  // OUTPUT
  for local_ctx in ctx.iter() {

    for &(i, ref line) in local_ctx.iter() { // <10> `ref line` informs the compiler that we wish to borrow this value, rather than move it. These two terms are explained fully later in later chapters.
      let line_num = i ;//+ 1;
      println!("{}: {}", line_num, line);
    }
  }
}

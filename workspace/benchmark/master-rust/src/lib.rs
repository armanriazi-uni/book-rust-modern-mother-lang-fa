#![allow(dead_code, unused_variables)]

/*
Main

## Commands

```rustup default nightly```

```cargo +nightly install racer```

```cargo bench --package master-rust-benches_lib --lib master-rust-benches-main -- bench_nothing_slowly --exact --nocapture```

```cargo doc  --package master-rust-benches_lib --message-format short --no-deps --open --color always```

```cargo test --doc  --package master-rust-benches_lib```

## What
`TODO`

## How
`TODO`

# Arguments

* `Arg1` - This is the [your type] to [your verb] the [your struct/func name]

# Return
`nothing`

## Example
 `TODO`

//```rust,compile_fail,no_run,ignore

*/



/*
#![feature(test)]
///The extern declaration is needed for crates internal to the compiler. In future versions of the compiler, this might not be needed and you will be able to use them like normal crates
extern crate test;

use test::Bencher;

pub fn do_nothing_slowly() {
    print!(".");
    for _ in 1..10_000_000 {};
}

pub fn do_nothing_fast() {
}

#[bench]
fn bench_nothing_slowly(b: &mut Bencher) {
    b.iter(|| do_nothing_slowly());
}

#[bench]
fn bench_nothing_fast(b: &mut Bencher) {
    b.iter(|| do_nothing_fast());
}
 */

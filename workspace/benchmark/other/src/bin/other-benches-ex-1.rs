#![allow(dead_code, unused_variables)]

/*
Main

## Commands


```cargo run --package other-benches_lib --bin other-benches-ex-1```

```cargo doc  --package other-benches_lib --message-format short --no-deps --open --color always```

```cargo test --doc  --package other-benches_lib```

## What
`medium-dyn-static-no-inline`

## How
`TODO`

# Arguments

* `Arg1` - This is the [your type] to [your verb] the [your struct/func name]

# Return
`nothing`

## Example
 `TODO`
*/
/// //```rust,compile_fail,no_run,ignore

use std::time::SystemTime;

struct PositiveBackend;

impl PositiveBackend{
    fn compute(&self, number: u64) -> u64{
        number+1
    }
}

fn main() {
    let backend = Box::new(PositiveBackend);
    let mut res= 0 as u64;
    let start_time = SystemTime::now();
    let total = 20_000_000 as u64;

    // our main loop
    for i in 0 .. total {
        res += backend.compute(i);
    }

    println!("Result: {}",res);
    println!("Elapsed_ms: {}", start_time.elapsed().unwrap().as_millis());
}

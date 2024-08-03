#![allow(dead_code, unused_variables)]

/// master-rust-lifetime-generic-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p  master-rust-lifetime-generic_bin --bin  master-rust-lifetime-generic-ex-1```
///
/// ## What
/// `lifetime_subtyping`
///
/// ## How
/// `This is read as the lifetime 'a outlives 'b or in other words 'b should never live longer than 'a.`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `unimplemented`
///
/// ## Example
/// `TODO`


struct Decoder<'a, 'b, S, R> {
    schema: &'a S,
    reader: &'b R
}

impl<'a, 'b, S, R> Decoder<'a, 'b, S, R>
where 'a: 'b {

}

fn main() {
    let a: Vec<u8> = vec![];
    let b: Vec<u8> = vec![];
    let decoder = Decoder {schema: &a, reader: &b};
}

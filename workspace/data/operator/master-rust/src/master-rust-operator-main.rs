#![allow(dead_code, unused_variables)]
/// Main
///
/// ## Commands
///
/// ```cargo run -q -p master_rust_operator_bin --bin master-rust-operator-main```
///
/// ```cargo doc  --package master_rust_operator_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package master_rust_operator_bin ```
///
/// ## What
/// `we are using a u8 as Rust does not have a native type to represent bit`
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

/// Implements a boolean `and` gate taking as input two bits and returning a bit as its
/// output
pub fn and(a: u8, b: u8) -> u8 {
    match (a,b) {
        (1, 1) => 1,
        _ => 0
    }
}

/// Implements a boolean `xor` gate taking as input two bits and returning a bit as its
/// output
pub fn xor(a: u8, b: u8) -> u8 {
    match (a,b) {
        (1, 0) | (0, 1) => 1,
        _ => 0
    }
}

#[cfg(test)]
mod tests {
    use crate::{xor, and};
    #[test]
    fn test_and() {
        assert_eq!(1, and(1, 1));
        assert_eq!(0, and(0, 1));
        assert_eq!(0, and(1, 0));
        assert_eq!(0, and(0,0));
    }

    #[test]
    fn test_xor() {
        assert_eq!(1, xor(1, 0));
        assert_eq!(0, xor(0, 0));
        assert_eq!(0, xor(1, 1));
        assert_eq!(1, xor(0, 1));
    }




    pub type Sum = u8;
    pub type Carry = u8;

    pub fn half_adder_input_output() -> Vec<((u8, u8), (Sum, Carry))> {
        vec![
            ((0, 0), (0, 0)),
            ((0, 1), (1, 0)),
            ((1, 0), (1, 0)),
            ((1, 1), (0, 1)),
        ]
    }

    /// This function implements a half adder using primitive gates
    fn half_adder(a: u8, b: u8) -> (Sum, Carry) {
        (xor(a, b), and(a, b))
    }

    #[test]
    fn one_bit_adder() {
        for (inn, out) in half_adder_input_output() {
            let (a, b) = inn;
            assert_eq!(half_adder(a, b), out);
        }
    }
}

fn main(){
    
}

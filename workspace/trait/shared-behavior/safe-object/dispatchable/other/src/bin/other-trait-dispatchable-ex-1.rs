#![allow(dead_code, unused_variables)]


/// other-trait-dispatchable-ex-1
///
/// ## Commands
///
/// ```cargo run -q -p other-trait-dispatchable_bin --bin  other-trait-dispatchable-ex-1```
///
/// ## What
/// `Calculate the distance covered by a flying specie`
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

struct Creatures{
     _time : i32,
}
impl Creatures {
    fn fly(&self) {
    }
    fn new(speed:&i32)-> Self{
        Creatures{
            _time: *speed
        }
    }
}
trait Bees {
    fn fly(&self);

}

trait Birds {
    fn fly(&self);

}
impl Bees for Creatures {
    fn fly(&self) {
        println!("Bee flies {} feet",&self._time*2);

    }
}

impl Birds for Creatures {
    fn fly(&self) {
        println!("Birds flies {} feet",&self._time);
    }
}
fn main() {
    let bees_ceatures = Creatures::new(&5);
    let birds_ceatures = Creatures::new(&5);
    Bees::fly(&bees_ceatures);
    Birds::fly(&birds_ceatures);


}

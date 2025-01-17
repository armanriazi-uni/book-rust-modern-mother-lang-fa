#![allow(dead_code, unused_variables)]

/// lpxxn-behavioral-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p lpxxn-behavioral_bin --bin lpxxn-behavioral-ex-3```
///
/// ```cargo doc  --package lpxxn-behavioral_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package lpxxn-behavioral_bin```
///
/// ## What
///`Iterator`
///
/// ## How
/// Iterator is a behavioral design pattern that lets you traverse elements of a collection without exposing its underlying representation (list, stack, tree, etc.).
///https://github.com/lpxxn/rust-design-pattern
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///
/// //```rust,compile_fail,no_run,ignore


trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
    fn current(&self) -> Option<T>;
    fn has_next(&self) -> bool;
    fn reset(&mut self);
}
struct Container<T> {
    data: Vec<T>,
}
struct ConcreteIterator<'a, T> {
    idx: usize,
    container: &'a Container<T>,
}
impl<'a, T: Clone> ConcreteIterator<'a, T> {
    fn new(container: &'a Container<T>) -> ConcreteIterator<T> {
        ConcreteIterator { idx: 0, container }
    }
}
impl<'a, T: Clone> Iterator<T> for ConcreteIterator<'a, T> {
    fn next(&mut self) -> Option<T> {
        let item = self.container.data.get(self.idx).cloned();
        self.idx += 1;
        item
    }
    fn current(&self) -> Option<T> {
        self.container.data.get(self.idx).cloned()
    }
    fn has_next(&self) -> bool {
        self.container.data.len() > self.idx
    }
    fn reset(&mut self) {
        self.idx = 0;
    }
}
impl<T: Clone> Container<T> {
    fn new() -> Container<T> {
        Container { data: Vec::new() }
    }
    fn add_item(&mut self, item: T) {
        self.data.push(item);
    }
    fn iter(&self) -> impl Iterator<T> + '_ {
        ConcreteIterator::new(self)
    }
}
fn main() {
    let mut c = Container::new();
    c.add_item(1);
    c.add_item(2);
    c.add_item(3);
    let mut iter = c.iter();
    let has_next = iter.has_next();
    assert_eq!(has_next, true);
    let item = iter.next();
    println!("item: {:?}", item);
    iter.reset();
    while iter.has_next() {
        let v = iter.next().unwrap();
        println!("item: {}", v);
    }
    let item = iter.next();
    assert_eq!(item, None);
}

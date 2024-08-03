#![allow(dead_code, unused_variables)]

/// lpxxn-structural-ex-3
///
/// ## Commands
///
/// ```cargo run -q -p lpxxn-structural_bin --bin lpxxn-structural-ex-3```
///
/// ```cargo doc  --package lpxxn-structural_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package lpxxn-structural_bin```
///
/// ## What
///`Proxy`
///
/// ## How
/// `TODO`
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


/// Proxy is a structural design pattern that lets you provide a substitute or placeholder for another object.
/// A proxy controls access to the original object, allowing you to perform something either before or after the request gets through to the original object.

///The Subject trait declares common operations for both RealSubject and
///the Proxy. As long as the client works with RealSubject using this
///interface, you'll be able to pass it a proxy instead of a real subject.
trait Subject {
    fn request(&self);
}
struct RealSubject {}
impl Subject for RealSubject {
    fn request(&self) {
        println!("RealSubject: handling request.");
    }
}
struct Proxy<'a> {
    real_subject: &'a RealSubject,
}
impl<'a> Proxy<'a> {
    fn new(real_subject: &'a RealSubject) -> Proxy {
        Proxy { real_subject }
    }
    fn check_access(&self) -> bool {
        println!("Proxy: checking access prior to firing a real request.");
        true
    }
    fn log_access(&self) {
        println!("Proxy: logging the request.");
    }
}
impl<'a> Subject for Proxy<'a> {

    fn request(&self) {
        if self.check_access() {
            self.real_subject.request();
            self.log_access();
        }
    }
}
struct Client;
impl Client {

    fn client_code<T: Subject>(subject: &T) {
        subject.request();
    }
}

fn main() {
    let real_subject = RealSubject {};
    println!("client: executing the client code with a real subject:");
    Client::client_code(&real_subject);

    println!("");
    println!("client: executing the same client code with a proxy:");
    let proxy = Proxy::new(&real_subject);
    Client::client_code(&proxy);
}

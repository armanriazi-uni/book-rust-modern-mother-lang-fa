
/// independant test
///
/// ## Commands
///
///
/// ```cd test_independent```
/// 
/// ```cargo test  -p test_independent -- --show-output --nocapture --test-threads 1```
///
/// ```cargo test  -p test_independent-- --show-output --ignore```
///             
/// ```cargo doc  --package integration_test--message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package test_independent```
///
/// ## What
/// `TODO`
///
/// ## How
/// `By default, cargo test runs tests in parallel on multiple threads. If your setup/teardown methods have side effects that aren't thread-safe, you might want to force single-threaded test execution with the --test-threads 1 flag`
///
/// # Arguments
///
/// * `TODO`
///
/// # Return
/// 
/// `passed`
/// 
mod user_model;


#[cfg(test)]
mod tests {
    use super::*;
    use user_model::*;


    #[test]
    pub fn test_user_get_name() {
        let ctx = setup();
        assert_eq!(ctx.user.get_name(), "Bob")
    }

    #[test]
    pub fn test_user_set_name() {
        let mut ctx = setup();
        ctx.user.set_name("Jane".to_string()); // no performance cost compared to String::from
        assert_eq!(ctx.user.get_name(), "Jane")
    }

    #[test]
    pub fn test_user_get_age() {
        let ctx = setup();
        assert_eq!(ctx.user.get_age(), 20)
    }

    #[test]
    pub fn test_user_set_age() {
        let mut ctx = setup();
        ctx.user.set_age(25);
        assert_eq!(ctx.user.get_age(), 25)
    }

    #[test]
    pub fn test_user_get_weight() {
        let ctx = setup();
        assert_eq!(ctx.user.get_weight(), 150.23)
    }

    #[test]
    pub fn test_user_set_weight() {
        let mut ctx = setup();
        ctx.user.set_weight(160.8);
        assert_eq!(ctx.user.get_weight(), 160.8)
    }
}
#[test]
#[ignore]
fn expensive_test() {
    //common::setup();
    assert_eq!(7, 7);
}

mod user_context;

pub fn setup() -> TestContext {
    println!("Test setup ...");

    TestContext {
       user: user_context::User::new(String::from("Bob"), 20, 150.23)
    }
}
pub struct TestContext {
   pub user: user_context::User,
}

 impl Drop for TestContext {
  fn drop(&mut self) {
        println!("Test teardown ...");
    }
}

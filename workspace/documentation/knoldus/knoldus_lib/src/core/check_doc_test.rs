#![crate_name = "doc"]

#[cfg(doctest)]
pub mod check_doc_test {

    pub fn check_doc_macro() {
        doc_comment::doctest!("../sample_doc.md");
    }

    #[cfg(doctest)]
    pub fn bar() {
        println!("hello");
    }

    // We can use parenthesis too, but not in rustc 1.44.1...

    doc_comment::doctest! { "../sample_doc.md" }
}

#[doc = include_str!("../sample_doc.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

/// We have a struct here. Remember it doesn't accept negative numbers!
pub struct MyStruct(pub usize);

/// ```rust,no_run,compile_fail,ignore
/// let x = my_crate::MyStruct(-5);
/// ```
#[cfg(doctest)]
pub struct MyStructOnlyTakesUsize;

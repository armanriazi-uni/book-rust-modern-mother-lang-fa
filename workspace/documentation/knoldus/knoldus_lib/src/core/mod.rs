//! Module containing shapes
//!
//! e.g paraph: Pipelines are the core data type used to poll or wait for frames from librealsense2. The
//! pipeline is constructed from a [`Context`](crate::context::Context), and streaming is started
//! by feeding the pipeline a configuration (see: [`Config`](crate::config::Config)).

pub mod cube;

pub mod square;

pub mod check_doc_test;

pub mod person;

/// mod
pub mod good {
    //! good
    ///
    pub struct Goodstruct;
}

/// mod
pub mod bad {
    /// bad
    ///
    pub struct Badstruct;
}

#[doc(inline)]
pub use bar::Bar;
/// bar docs
mod bar {
    /// the docs for Bar
    pub struct Bar;
}

#[doc(no_inline)]
pub use foo::Foo;

/// bar docs
mod foo {
    /// the docs for Foo
    pub struct Foo;
}

#[doc(hidden)]
pub use poo::Poo;

/// poo docs
mod poo {
    /// the docs for Poo
    pub struct Poo;
}


#[doc(alias = "big")]
pub struct BigX;

//! Rust bindings for [unqlite][] library.
//!
//! **unqlite** is
//! > An Embeddable NoSQL Database Engine
//!
//! [unqlite]: http://unqlite.org
extern crate libc;

#[allow(dead_code, non_snake_case, non_camel_case_types)]
mod bindgen;
#[allow(dead_code)]
mod constants;

pub use self::bindgen::*;
pub use self::constants::*;

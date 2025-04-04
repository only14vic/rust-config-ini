#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate core;
extern crate alloc;

//#[cfg(not(feature = "std"))]
//mod no_std;

mod binds;
mod ini;

pub use ini::*;

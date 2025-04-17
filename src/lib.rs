#![cfg_attr(not(feature = "std"), no_std)]

#[allow(unused_imports)]
#[macro_use]
extern crate core;
extern crate alloc;

mod binds;
mod ini;

#[cfg(not(feature = "std"))]
pub mod no_std;
pub mod base;
pub mod logger;

pub use {ini::*, set_from_iter_derive::*};

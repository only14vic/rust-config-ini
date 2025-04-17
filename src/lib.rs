#![cfg_attr(not(feature = "std"), no_std)]

#[allow(unused_imports)]
#[macro_use]
extern crate core;
extern crate alloc;

#[cfg(not(feature = "std"))]
pub use libc_print::std_name::*;

mod binds;
mod ini;

pub mod base;
pub mod logger;

pub use {ini::*, set_from_iter_derive::*};

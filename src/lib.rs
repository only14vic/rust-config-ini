#![cfg_attr(not(feature = "std"), no_std)]

#[allow(unused_imports)]
#[macro_use]
extern crate core;
extern crate alloc;

//#[cfg(not(feature = "std"))]
//#[allow(unused_imports)]
//pub use libc_print::std_name::*;

#[cfg(not(feature = "std"))]
mod no_std;

mod binds;
mod ini;

pub mod base;

pub use {ini::*, set_from_iter_derive::*};

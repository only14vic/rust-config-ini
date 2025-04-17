#![cfg_attr(not(feature = "std"), no_std)]
#![no_main]

//#[cfg(not(feature = "std"))]
//include!("../src/no_std.rs");

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;
extern crate core;

#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use libc_print::std_name::*;
use {
    alloc::{
        boxed::Box,
        string::{String, ToString},
        vec::Vec
    },
    config_ini::{base::BaseFromInto, Ini, SetFromIter},
    core::{ffi::c_int, hint::black_box, num::NonZero, str::FromStr, usize},
    serde::Serialize,
    yansi::Paint
};

#[derive(Default, Debug, Serialize, SetFromIter)]
pub struct Config {
    version: f32,
    general: General
}

#[derive(Default, Debug, PartialEq, Serialize)]
pub enum Lang {
    #[default]
    Ru,
    En
}

impl FromStr for Lang {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ru" => Ok(Self::Ru),
            "en" => Ok(Self::En),
            _ => Err("Invalid value".to_string())
        }
    }
}

#[derive(Default, Debug, Serialize, SetFromIter)]
pub struct General {
    #[parse]
    str: Option<Box<Lang>>,
    number: u32,
    boolean: bool,
    list: Vec<u32>,
    text: String,
    foo: Foo
}

#[derive(Default, Debug, Serialize, SetFromIter)]
pub struct Foo {
    #[parse]
    str: Lang,
    number: Option<NonZero<u32>>,
    boolean: Option<bool>,
    text: Box<str>
}

const MAX_ITERS: usize = 100_000;
const FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/config.ini");

#[no_mangle]
fn main() -> c_int {
    let mut config = Config::default();

    for _ in 0..MAX_ITERS {
        black_box({
            let ini = Ini::from_file(&FILE_PATH).unwrap();
            config.set_from_iter(&ini).unwrap();
        });
    }

    let config_json = config.to_json().unwrap();

    println!(
        "Struct {}\nJSON {}",
        format!("{:#?}", &config).bright_blue().italic(),
        format!("{:#?}", &config_json).blue().italic()
    );
    println!("Max iters: {}", MAX_ITERS.bright_red().bold());
    println!(
        "{}\n{}",
        format!("no_std = {}", cfg!(not(feature = "std")))
            .red()
            .on_green(),
        format!("static = {}", cfg!(target_env = "musl"))
            .blue()
            .on_bright_green()
    );

    #[cfg(not(target_env = "musl"))]
    unsafe {
        libc::malloc_stats()
    };

    return libc::EXIT_SUCCESS;
}

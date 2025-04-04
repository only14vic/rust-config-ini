#![cfg_attr(not(feature = "std"), no_std)]
#![no_main]

#[cfg(not(feature = "std"))]
include!("../src/no_std.rs");

extern crate core;
extern crate alloc;
extern crate config_ini;
extern crate set_from_iter_derive;

use {
    alloc::{
        boxed::Box,
        string::{String, ToString},
        vec::Vec
    },
    config_ini::Ini,
    core::{ffi::c_int, num::NonZero, str::FromStr},
    libc::EXIT_SUCCESS,
    set_from_iter_derive::SetFromIter
};

#[derive(Default, Debug, SetFromIter)]
pub struct Config {
    version: f32,
    general: ConfigGeneral
}

#[derive(Default, Debug, PartialEq)]
pub enum ConfigEnum {
    #[default]
    Ru,
    En
}

impl FromStr for ConfigEnum {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ru" => Ok(Self::Ru),
            "en" => Ok(Self::En),
            _ => Err("Invalid value".to_string())
        }
    }
}

#[derive(Default, Debug, SetFromIter)]
pub struct ConfigGeneral {
    #[parse]
    str: Option<Box<ConfigEnum>>,
    number: u32,
    boolean: bool,
    list: Vec<u32>,
    text: String,
    foo: ConfigFoo
}

#[derive(Default, Debug, SetFromIter)]
pub struct ConfigFoo {
    #[parse]
    str: ConfigEnum,
    number: Option<NonZero<u32>>,
    boolean: Option<bool>,
    text: Box<str>
}

#[no_mangle]
fn main() -> c_int {
    let file_path = env!("CARGO_MANIFEST_DIR").to_string() + "/examples/config.ini";

    let ini = Ini::from_file(&file_path).unwrap();
    let mut config = Config::default();

    config.set_from_iter(&ini).unwrap();

    assert_eq!(config.general.str, Some(ConfigEnum::En.into()));
    dbg!(ini, config);

    return EXIT_SUCCESS;
}

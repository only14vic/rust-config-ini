use {
    crate::{
        alloc::string::ToString,
        base::{ok, Void}
    },
    alloc::string::String,
    core::ffi::{c_int, CStr},
    libc::getenv,
    log::{Level, LevelFilter, Log, ParseLevelError},
    yansi::Paint
};
#[cfg(not(feature = "std"))]
use libc_print::std_name::*;

static LOGGER: Logger = Logger;

#[cfg(debug_assertions)]
const LEVEL_DEFAULT: LevelFilter = LevelFilter::Debug;
#[cfg(not(debug_assertions))]
const LEVEL_DEFAULT: LevelFilter = LevelFilter::Info;

pub struct Logger;

#[no_mangle]
pub extern "C" fn log_init() -> c_int {
    match Logger::init() {
        Ok(..) => 0,
        Err(e) => {
            eprintln!("ERROR: log_init() - {e}");
            -1
        }
    }
}

impl Logger {
    pub fn init() -> Void {
        let level: LevelFilter = unsafe {
            match getenv(c"LOG_LEVEL".as_ptr()) {
                level if level.is_null() == false => {
                    let level = CStr::from_ptr(level).to_string_lossy();
                    if level.is_empty() {
                        LEVEL_DEFAULT
                    } else {
                        level
                            .trim()
                            .parse()
                            .map_err(|e: ParseLevelError| e.to_string())?
                    }
                },
                _ => LEVEL_DEFAULT
            }
        };

        log::set_logger(&LOGGER).unwrap();
        log::set_max_level(level);
        log::debug!("LOG_LEVEL: {level}");

        ok()
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let level_colored: String = match record.level() {
                l @ Level::Info => l.bright_blue().to_string(),
                l @ Level::Warn => l.bright_yellow().to_string(),
                l @ Level::Error => l.bright_red().to_string(),
                l @ Level::Trace => l.bright_black().to_string(),
                l @ Level::Debug => l.to_string()
            };
            eprintln!(
                "[{}] [{}] {}",
                level_colored,
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

use core::fmt;
use log::{self, Level, Log, Record, Metadata};

use crate::println;

pub static SIMPLE_LOGGER : SimpleLogger = SimpleLogger;

pub struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        print_in_color(
            format_args!(
                "[{:>5}][-] {}",
                record.level(),
                record.args()
            )
            , level_to_color_code(record.level()))
    }

    fn flush(&self) {
        
    }
}

fn print_in_color(args: fmt::Arguments, color_code: u8) {
    println!("\x1b[{}m{}\x1b[0m", color_code, args)
}

fn level_to_color_code(level : Level) -> u8 {
    match level {
        Level::Error => 31,
        Level::Warn => 93,
        Level::Info => 34,
        Level::Debug => 32,
        Level::Trace => 90,
    }
}
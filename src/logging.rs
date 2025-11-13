extern crate log;
extern crate xdg;
use log::*;
use simplelog::*;
use std::fs::File;

// Creates a constant string for our package name.
const PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");

pub fn initialize_logging() {
    // Creates a 'xdg' Structure to BaseDirectories loaded with our package name. ¿Unwrap to panic if null?
    let xdg_dirs = xdg::BaseDirectories::with_prefix(PROGRAM_NAME).unwrap();
    // Creates a cache log file in the cache path provided from BaseDirectories.
    let log_file_path = xdg_dirs
        .place_cache_file(format!("{}.log", PROGRAM_NAME))
        .unwrap();
    // Initiates the 'simplelog' CombinedLogger to create [2] loggers; a terminal logger & a write logger.
    CombinedLogger::init(vec![
        // Creates a terminal logger setting it to information level, default structure config, mixed terminal mode (Use Stderr for Errors and Stdout otherwise), automatic color (Try to use colors, but don’t force the issue.).
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        // Creates a write logger setting it to information level, default structure config, create the cache log file 'log_file_path'.
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(log_file_path).unwrap(),
        ),
    ])
    .unwrap();
}

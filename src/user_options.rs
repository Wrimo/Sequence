use std::sync::Mutex;

pub struct Options {
    pub debug: bool,
}

pub static USER_OPTIONS: Mutex<Options> = Mutex::new(Options { debug: false });

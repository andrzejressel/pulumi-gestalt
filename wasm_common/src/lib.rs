use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;

use crate::logger::Logger;

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
mod bindings;
mod logger;

static IS_SET : AtomicBool = AtomicBool::new(false);
static LOGGER: Logger = Logger {};

pub fn setup_logger() {
    if IS_SET.swap(true, Relaxed) {
        return;
    }
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}
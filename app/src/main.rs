#![no_main]
#![no_std]

use cortex_m::interrupt;
use cortex_m_semihosting::{
    debug,
    hio::{self, HStdout},
};

#[allow(unused)]
use stlog::{global_logger, GlobalLog, error, warn, info, debug, trace};
use rt::entry;

struct Logger;

entry!(main);

fn main() -> ! {
    error!("Hello, world!");

    warn!("Goodbye");

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

impl GlobalLog for Logger {
    fn log(&self, address: u8) {
        // we use a critical section (`interrupt::free`) to make the access to the
        // `static mut` variable interrupt safe which is required for memory safety
        interrupt::free(|_| unsafe {
            static mut HSTDOUT: Option<HStdout> = None;

            // lazy initialization
            if HSTDOUT.is_none() {
                HSTDOUT = Some(hio::hstdout()?);
            }

            let hstdout = HSTDOUT.as_mut().unwrap();

            hstdout.write_all(&[address])
        }).ok(); // `.ok()` = ignore errors
    }
}

#[global_logger]
static LOGGER: Logger = Logger {};
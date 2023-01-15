#![no_main]
#![no_std]

use cortex_m_semihosting::{
    debug,
    hio,
};

use rt::entry;
#[allow(unused)]
use stlog::{error, warn, info, debug, trace, Log, GlobalLog, global_logger};

// fn foo() {
//     info!("Hello!");
//     error!("Hey!");
//     info!("Bye!");
//     warn!("A warning: Watch out!");
//     debug!("A diagnostic piece of information");
//     trace!("You Are Here!");
//     // info!("Hey!"); //~ ERROR symbol `Hey!` is already defined
//     // warn!("Hey!"); //~ ERROR symbol `Bye!` is already defined
// }

entry!(main);

fn main() -> ! {
    warn!("Hello, world!");
    
    error!("Goodbye"); // <- CHANGED!

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

struct Logger;

impl Log for Logger {
    type Error = ();

    fn log(&mut self, address: u8) -> Result<(), Self::Error> {
        let mut local = hio::hstdout().unwrap();
        local.write_all(&[address])    
    }
}

impl GlobalLog for Logger {
    fn log(&self, address: u8) {
        let mut local = hio::hstdout().unwrap();
        local.write_all(&[address]).unwrap();
    }
}

#[global_logger]
static LOGGER: Logger = Logger;

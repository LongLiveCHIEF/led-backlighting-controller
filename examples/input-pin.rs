#![no_main]
#![no_std]

use panic_rtt_target as _;
use xiao_m0::{self as hal, entry, pac, Pins};

use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::gpio::v2::{Pin, FloatingInput, PullDownInput, PA11};
use pac::Peripherals;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let pins = hal::gpio::v2::Pins::new(peripherals.PORT);
    let pa11: Pin<PA11, PullDownInput>  = pins.pa11.into_pull_down_input();

    rtt_init_print!();
    rprintln!("Initialization complete");

    loop {
        continue;
    }
}

#![no_main]
#![no_std]

extern crate xiao_m0 as hal;
use panic_rtt_target as _;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [TCC0])]
mod app {
    use hal::{
        clock::{
            ClockGenId, ClockSource, GenericClockController
        },
        gpio::{Floating, Input, Pa8, PullDown, PullUp},
        prelude::*,
        Pins,
        rtc::{Count32Mode, Rtc},
    };
    use rtic::time::{duration::Milliseconds, Instant};
    use rtt_target::{rprintln, rtt_init_print};

    #[resources]
    struct Resources {
        modeDetectPin: Pa8<Input<PullDown>>,
    }

    #[init()]
    fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {
        let mut peripherals = cx.device;
        let mut pins = Pins::new(peripherals.PORT);

        let mut modeDetectPin = pins.a4.into_pull_down_input(&mut pins.port);

        rtt_init_print!();
        rprintln!("Initialization Complete!");
        ( init::LateResources {modeDetectPin}, init::Monotonics())
    }

    // Optional idle task, if left out idle will be a WFI.
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("Hello from idle!");

        loop {
            // Do some work or WFI.
            continue;
        }
    }
}


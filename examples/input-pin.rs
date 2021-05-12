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
        gpio::{Floating, Input, Pa8, PullDown},
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

    #[monotonic(binds = RTC, default = true)]
    type RtcMonotonic = Rtc<Count32Mode>;

    #[init()]
    fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {
        let mut peripherals = cx.device;
        let mut pins = Pins::new(peripherals.PORT);
        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );

        // initialize monotonic
        let rtc_clock_src = clocks
            .configure_gclk_divider_and_source(ClockGenId::GCLK2, 1, ClockSource::XOSC32K, false)
            .unwrap();
        clocks.configure_standby(ClockGenId::GCLK2, true);
        let rtc_clock = clocks.rtc(&rtc_clock_src).unwrap();
        let rtc = Rtc::count32_mode(peripherals.RTC, rtc_clock.freq(), &mut peripherals.PM);

        let mut modeDetectPin = pins.a4.into_pull_down_input(&mut pins.port);

        rtt_init_print!();
        rprintln!("Initialization Complete!");
        ( init::LateResources {modeDetectPin}, init::Monotonics(rtc))
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


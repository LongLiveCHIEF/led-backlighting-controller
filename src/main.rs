#![no_main]
#![no_std]

extern crate xiao_m0 as hal;
use panic_halt as _;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [EVSYS])]
mod app {
    use rtt_target::{ rprintln, rtt_init_print };
    use hal::pac::Peripherals;
    use hal::clock::{ClockGenId, ClockSource, GenericClockController};
    use hal::rtc::{Count32Mode, Rtc};
    use rtic_monotonic::Extensions;

    #[resources]
    struct Resources {
    led:
        hal::gpio::Pin<hal::gpio::v2::PA17, hal::gpio::v2::Output<hal::gpio::v2::PushPull>> ,
    }

    #[monotonic(binds = RTC, default = true)]
    type RtcMonotonic = Rtc<Count32Mode>;

    #[init()]
    fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {
        let mut peripherals: Peripherals = cx.device;
        let mut pins = hal::Pins::new(peripherals.PORT);
        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let _gclk = clocks.gclk0();
        let rtc_clock_src = clocks
            .configure_gclk_divider_and_source(ClockGenId::GCLK2, 1, ClockSource::XOSC32K, false)
            .unwrap();
        clocks.configure_standby(ClockGenId::GCLK2, true);
        let rtc_clock = clocks.rtc(&rtc_clock_src).unwrap();
        let rtc = Rtc::count32_mode(peripherals.RTC, rtc_clock.freq(), &mut peripherals.PM);

        let led = pins.led0.into_open_drain_output(&mut pins.port);

        rtt_init_print!();
        rprintln!("Initialization complete");

        blink::spawn().unwrap();

        ( init::LateResources { led, }, init::Monotonics(rtc))
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

    #[task(resources = [led])]
    fn blink(mut cx: blink::Context) {
        cx.resources.led.lock(|led| led.toggle());
        rprintln!("toggling led");
        blink::spawn_after(1_u32.seconds()).ok();
    }
}


#![no_main]
#![no_std]

extern crate xiao_m0 as hal;
use panic_halt as _;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [EVSYS])]
mod app {
    use hal::clock::{ClockGenId, ClockSource, GenericClockController};
    use hal::rtc::{Count32Mode, Rtc};
    use hal::time::MegaHertz;
    use hal::spi_master;
    use hal::gpio::{Pa5, Pa6, Pa7, PfD};
    use hal::sercom::{Sercom0Pad1, Sercom0Pad2, Sercom0Pad3};
    use hal::prelude::*;
    use rtic_monotonic::Extensions;
    use rtt_target::{rprintln, rtt_init_print};
    use ws2812_spi::Ws2812 as ws2812;
    use smart_leds::hsv::{hsv2rgb, Hsv};
    use smart_leds::SmartLedsWrite;

    #[monotonic(binds = RTC, default = true)]
    type RtcMonotonic = Rtc<Count32Mode>;

    #[resources]
    struct Resources {
        ledString: ws2812<hal::sercom::SPIMaster0<Sercom0Pad1<Pa5<PfD>>, Sercom0Pad2<Pa6<PfD>>, Sercom0Pad3<Pa7<PfD>>>>,
    }

    #[init()]
    fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {
        let mut peripherals = cx.device;
        let mut pins = hal::Pins::new(peripherals.PORT);
        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );

        // initialize monotonic
        let _gclk = clocks.gclk0();
        let rtc_clock_src = clocks
            .configure_gclk_divider_and_source(ClockGenId::GCLK2, 1, ClockSource::XOSC32K, false)
            .unwrap();
        clocks.configure_standby(ClockGenId::GCLK2, true);
        let rtc_clock = clocks.rtc(&rtc_clock_src).unwrap();
        let rtc = Rtc::count32_mode(peripherals.RTC, rtc_clock.freq(), &mut peripherals.PM);

        let spi = spi_master(
            &mut clocks,
            MegaHertz(3),
            peripherals.SERCOM0,
            &mut peripherals.PM,
            pins.a8,
            pins.a10,
            pins.a9,
            &mut pins.port
        );
        let ledString = ws2812::new(spi);

        rtt_init_print!();
        rprintln!("Initialization complete!");
        rainbow::spawn().unwrap();

        ( init::LateResources { ledString }, init::Monotonics(rtc))
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

    #[task(resources = [ledString])]
    fn rainbow(mut cx: rainbow::Context) {
        cx.resources.ledString.lock(|ledString| {
            for j in 0..255u8 {
                let colors = [hsv2rgb(Hsv {
                    hue: j,
                    sat: 255,
                    val: 2,
                })];
                ledString.write(colors.iter().cloned()).unwrap();
            }
        });
        rprintln!("Setting colors");
        rainbow::spawn_after(1_u32.seconds()).ok();
    }
}


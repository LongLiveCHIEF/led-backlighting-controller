#![no_main]
#![no_std]

extern crate xiao_m0 as hal;
use panic_halt as _;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [TCC0])]
mod app {
    use hal::clock::{ClockGenId, ClockSource, GenericClockController};
    use hal::rtc::{Count32Mode, Rtc};
    use hal::time::MegaHertz;
    use hal::spi_master;
    use hal::gpio::{ Input, Floating, Pin, Pb8, Pa5, Pa6, Pa7, PfD, PfA};
    use hal::eic::{pin::{Sense, ExtInt8}, EIC};
    use hal::sercom::{Sercom0Pad1, Sercom0Pad2, Sercom0Pad3};
    use hal::prelude::*;
    use rtic_monotonic::Extensions;
    use rtt_target::{rprintln, rtt_init_print};
    use ws2812_spi::Ws2812 as ws2812;
    use smart_leds::{brightness, RGB8, SmartLedsWrite, colors::BLUE as COLOR};

    const NUM_LEDS: usize = 20;

    #[monotonic(binds = RTC, default = true)]
    type RtcMonotonic = Rtc<Count32Mode>;

    #[resources]
    struct Resources {
        ledString: ws2812<hal::sercom::SPIMaster0<Sercom0Pad1<Pa5<PfD>>, Sercom0Pad2<Pa6<PfD>>, Sercom0Pad3<Pa7<PfD>>>>,
        button: ExtInt8<Pb8<PfA>>,
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
        let rtc_clock_src = clocks
            .configure_gclk_divider_and_source(ClockGenId::GCLK2, 1, ClockSource::XOSC32K, false)
            .unwrap();
        clocks.configure_standby(ClockGenId::GCLK2, true);
        let rtc_clock = clocks.rtc(&rtc_clock_src).unwrap();
        let rtc = Rtc::count32_mode(peripherals.RTC, rtc_clock.freq(), &mut peripherals.PM);

        // setup led control SPI
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

        // initialize color control button
        let gclk0 = clocks.gclk0();
        let eic_clock = clocks.eic(&gclk0).unwrap();
        let mut eic = EIC::init(&mut peripherals.PM, eic_clock, peripherals.EIC);
        let mut button = pins.a6.into_ei(&mut pins.port);
        button.sense(&mut eic, Sense::HIGH);
        button.enable_interrupt(&mut eic);

        rtt_init_print!();
        rprintln!("Initialization complete!");
        set_solid_color::spawn().unwrap();

        ( init::LateResources { ledString, button }, init::Monotonics(rtc))
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
    fn set_solid_color(mut cx: set_solid_color::Context) {
        cx.resources.ledString.lock(|ledString| {
            let leds: [RGB8; NUM_LEDS] = [COLOR; NUM_LEDS];
            ledString.write(brightness(leds.iter().cloned(), 8)).unwrap();
        });
        rprintln!("leds set to {}", COLOR);
    }

    #[task(resources = [ledString])]
    fn clear_leds(mut cx: clear_leds::Context) {
        cx.resources.ledString.lock(|ledString| {
            ledString.write([RGB8::default(); NUM_LEDS].iter().cloned()).unwrap();
        });
        rprintln!("leds cleared");
    }

    #[task(binds = EIC, resources=[button], priority = 2)]
    fn boop(mut cx: boop::Context) {
        cx.resources.button.lock(|button| {
            rprintln!("button pressed");
            button.clear_interrupt();
        })
    }
}

#![no_main]
#![no_std]

extern crate xiao_m0 as hal;
use panic_rtt_target as _;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [TCC0])]
mod app {
    use core::convert::TryInto;
    use hal::clock::{ClockGenId, ClockSource, GenericClockController};
    use hal::rtc::{Count32Mode, Rtc};
    use hal::spi_master;
    use hal::gpio::{ Input, Floating, Pb8, Pa5, Pa6, Pa7, Pa11, PfD, PfA};
    use hal::eic::{pin::{Sense, ExtInt8}, EIC};
    use hal::sercom::{Sercom0Pad1, Sercom0Pad2, Sercom0Pad3};
    use hal::prelude::*;
    use heapless::Vec;
    use rtic::time::{duration::Milliseconds, Instant};
    use rtt_target::{rprintln, rtt_init_print};
    use ws2812_spi::Ws2812 as ws2812;
    use smart_leds::{SmartLedsWrite, colors::{RED, GREEN, BLUE}};

    #[monotonic(binds = RTC, default = true)]
    type RtcMonotonic = Rtc<Count32Mode>;

    #[resources]
    struct Resources {
        ledString: ws2812<hal::sercom::SPIMaster0<Sercom0Pad1<Pa5<PfD>>, Sercom0Pad2<Pa6<PfD>>, Sercom0Pad3<Pa7<PfD>>>>,
        button: ExtInt8<Pb8<PfA>>,
        modeDetectPin: Pa11<Input<Floating>>,
        colors: core::iter::Cycle<<heapless::Vec<smart_leds::RGB<u8>, 3_usize> as IntoIterator>::IntoIter>,
    }

    #[init]
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
            2600.khz(),
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
        button.sense(&mut eic, Sense::BOTH);
        button.enable_interrupt(&mut eic);

        let modeDetectPin = pins.a3.into_floating_input(&mut pins.port);

        let mut colorCollection: Vec<smart_leds::RGB8, 3> = Vec::new();
        colorCollection.push(RED).unwrap();
        colorCollection.push(GREEN).unwrap();
        colorCollection.push(BLUE).unwrap();
        let colors = colorCollection.into_iter().cycle();
        writeLeds::spawn().unwrap();
        rtt_init_print!();
        rprintln!("Initialization complete!");

        ( init::LateResources { ledString, button, modeDetectPin, colors }, init::Monotonics(rtc)) }

      // Optional idle task, if left out idle will be a WFI.
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("Hello from idle!");

        loop {
            // Do some work or WFI.
            continue;
        }
    }

    #[task(binds = EIC, resources=[button], priority = 2)]
    fn onButtonInterrupt(mut cx: onButtonInterrupt::Context) {
        cx.resources.button.lock(|button| button.clear_interrupt());
        debounce::spawn_after(Milliseconds(30_u32)).ok();
    }

    #[task(resources = [modeDetectPin])]
    fn debounce(mut cx: debounce::Context){
        static mut HOLD: Option<hold::SpawnHandle> = None;
        static mut PRESSED_AT: Option<Instant<RtcMonotonic>> = None;
        if let Some(handle) = HOLD.take() {
            handle.cancel().ok();
        }

        if cx.resources.modeDetectPin.lock(|b| b.is_high().unwrap()) {
            PRESSED_AT.replace(monotonics::RtcMonotonic::now());
            *HOLD = hold::spawn_after(Milliseconds(1000u32)).ok();
        } else {
            if PRESSED_AT
                .take()
                .and_then(|i| monotonics::RtcMonotonic::now().checked_duration_since(&i))
                .and_then(|d| d.try_into().ok())
                .map(|t: Milliseconds<u32>| t < Milliseconds(1000_u32))
                .unwrap_or(false) {
                    rprintln!("changing mode value");
                    writeLeds::spawn().unwrap();
                }
        }
    }

    #[task(resources = [modeDetectPin])]
    fn hold(mut cx: hold::Context){
        if cx.resources.modeDetectPin.lock(|b| b.is_high().unwrap()) {
            rprintln!("selecting mode");
        }
    }

    #[task(resources = [ledString, colors])]
    fn writeLeds(cx: writeLeds::Context){
        let writeLeds::Resources {ledString, colors } = cx.resources;
        (ledString, colors).lock(|leds, colors| {
            leds.write(colors.next().iter().cloned()).unwrap()
        });
    }
}

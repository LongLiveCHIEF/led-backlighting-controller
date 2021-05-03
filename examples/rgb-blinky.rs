#![no_std]
#![no_main]

use panic_halt as _;

use xiao_m0 as hal;
use ws2812_spi as ws2812;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::TimerCounter;

use crate::ws2812::Ws2812;
use smart_leds::SmartLedsWrite;
use smart_leds_trait::RGB8;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = crate::hal::Pins::new(peripherals.PORT);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.PM);
    timer.start(8.khz());

    let spi = crate::hal::spi_master(
        &mut clocks,
        3.mhz(),
        peripherals.SERCOM0,
        &mut peripherals.PM,
        pins.a8,
        pins.a10,
        pins.a9,
        &mut pins.port,
    );
    const NUM_LEDS: usize = 19;
    let mut data: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    let empty: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    let mut ws = Ws2812::new(spi);
    loop {
        for j in 0..NUM_LEDS {
            data[j] = RGB8 {
                r: 0,
                g: 0,
                b: 0x10,
            }
        }
        ws.write(data.iter().cloned()).unwrap();
        delay.delay_ms(1000 as u16);
        ws.write(empty.iter().cloned()).unwrap();
        delay.delay_ms(1000 as u16);
    }
}

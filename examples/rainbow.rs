#![no_std]
#![no_main]

#[allow(unused)]
use panic_halt;

extern crate xiao_m0 as hal;
extern crate ws2812_spi as ws2812;
extern crate embedded_hal;
extern crate cortex_m_rt;
extern crate smart_leds;

use hal::clock::GenericClockController;
use hal::pac::*;
use hal::pac::{Peripherals, CorePeripherals};
use hal::delay::Delay;
use hal::time::MegaHertz;
use hal::spi_master;
use hal::gpio::{Pa5, Pa6, Pa7, PfD};
use hal::sercom::{Sercom0Pad1, Sercom0Pad2, Sercom0Pad3};
use hal::prelude::*;
use embedded_hal::blocking::delay::DelayMs;
use cortex_m_rt::entry;

use smart_leds::SmartLedsWrite;
use smart_leds_trait::RGB8;
use smart_leds::{brightness,hsv::{hsv2rgb, Hsv}};
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);

    let gclk0 = clocks.gclk0();
    let neopixel_spi = spi_master(
        &mut clocks,
        3_000_000.hz(),
        peripherals.SERCOM0,
        &mut peripherals.PM,
        pins.a8,
        pins.a10,
        pins.a9,
        &mut pins.port
    );
    let mut neopixel = ws2812::Ws2812::new(neopixel_spi);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    const NUM_LEDS: usize = 20;
    let mut data: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    let empty: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    rtt_init_print!();
    loop {
        data[0] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[1] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[2] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[3] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[4] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[5] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[6] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[7] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[8] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[9] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[10] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[11] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[12] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[13] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[14] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[15] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[16] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[17] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[18] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        data[19] = RGB8 {
            r: 0,
            g: 0,
            b: 0x10,
        };
        rprintln!("writing BLUE");
        neopixel.write(data.iter().cloned()).unwrap();
        delay.delay_ms(1000 as u16);
        rprintln!("clearning leds");
        neopixel.write(empty.iter().cloned()).unwrap();
        delay.delay_ms(1000 as u16);
    }
}

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into()
    }
    if wheel_pos < 170 {
        wheel_pos -=85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into()
    }
    wheel_pos -= 170;
    (wheel_pos*3, 255 - wheel_pos * 3, 0).into()
}

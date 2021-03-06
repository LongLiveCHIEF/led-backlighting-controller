#![no_std]
#![no_main]

use panic_rtt_target as _;

use xiao_m0 as hal;
use ws2812_spi as ws2812;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::TimerCounter;

use crate::ws2812::Ws2812;
use smart_leds::{brightness, SmartLedsWrite};
use smart_leds_trait::RGB8;

use cortex_m_rt::entry;

use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
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
    const NUM_LEDS: usize = 20;
    let mut data: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    let mut ws = Ws2812::new(spi);
    loop {
        for j in 0..(256*5) {
            for i in 0..NUM_LEDS {
                data[i] = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
            }
        }
        ws.write(brightness(data.iter().cloned(), 32)).unwrap();
        rprintln!("colors set");
        delay.delay_ms(5u8);
    }
}

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

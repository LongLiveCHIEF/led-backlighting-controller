#![no_main]
#![no_std]

extern crate xiao_m0 as hal;
use panic_halt as _;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [EVSYS])]
mod app {
    use hal::gpio::v2::{Pins, FloatingInput, PB08};
    use rtt_target::{rprintln, rtt_init_print};

    #[resources]
    struct Resources {
        button: PB08<FloatingInput>
    }

    #[init()]
    fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {
        let mut peripherals = cx.device;
        let mut pins = Pins::new(peripherals.PORT);

        let mut button = pins.pb08.into_floating_input();
        button.enable_interrupt();
        button.enable_event();

        rtt_init_print!()
        rprintln!("Initialization Complete!");
        ( init::LateResources {button,}, init::Monotonics())
    }

    #[task(resources = [button])]
    fn boop(cs: boop::Context) {
        rprintln!("boop!");
        cx.resources.button.lock(|button| {
            button.clear_interrupt();
        })
    }
}



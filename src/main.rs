#![no_main]
#![no_std]

use stm32f3xx_hal as _;
use panic_halt as _;

#[rtic::app(device = stm32f3xx_hal::stm32, 
  peripherals = true, 
  dispatchers=[USART1_EXTI25])]
mod app {
  use rtt_target::{ rprintln, rtt_init_print };
  use stm32f3xx_hal::{
    gpio::{gpiob::PB13, Output, PushPull},
    prelude::*
  };

  #[resources]
  struct Resources {
    led: PB13<Output<PushPull>>,
  }

  #[init]
  fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {

    let pac = cx.device;
    let mut rcc = pac.RCC.constrain();
    let mut gpiob = pac.GPIOB.split(&mut rcc.ahb);

    let mut led = gpiob.pb13.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    led.set_low().ok();

    rtt_init_print!();
    rprintln!("Initialization complete");
   ( init::LateResources { led, }, init::Monotonics())
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

  #[task]
  fn hello(_: hello::Context) {
    rprintln!("Hello, world!");
  }
}

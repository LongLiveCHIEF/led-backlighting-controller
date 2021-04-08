#![no_main]
#![no_std]

use stm32f3xx_hal as _;
use panic_halt as _;

#[rtic::app(device = stm32f3xx_hal::stm32, dispatchers=[USART1_EXTI25])]
mod app {
  use rtt_target::{ rprintln, rtt_init_print };
  #[init]
  fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {
    let _device: stm32f3xx_hal::stm32::Peripherals = cx.device;
    rtt_init_print!();
    rprintln!("Initialized...");
   ( init::LateResources {}, init::Monotonics())
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

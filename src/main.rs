#![no_std]
#![no_main]

extern crate panic_halt;

use embedded_hal as emb;
use xiao_m0 as bsp;

use bsp::{hal, pac};
use emb::{digital::v2::ToggleableOutputPin, prelude::*};

#[bsp::entry]
fn main() -> ! {
    let cp = pac::CorePeripherals::take().unwrap();
    let mut dp = pac::Peripherals::take().unwrap();

    let mut clocks = hal::clock::GenericClockController::with_external_32kosc(
        dp.GCLK,
        &mut dp.PM,
        &mut dp.SYSCTRL,
        &mut dp.NVMCTRL,
    );

    let pins = bsp::Pins::new(dp.PORT);
    let mut delay = hal::delay::Delay::new(cp.SYST, &mut clocks);
    let mut led = pins.led0.into_push_pull_output();

    loop {
        led.toggle().ok();
        delay.delay_ms(1000u16);
    }
}

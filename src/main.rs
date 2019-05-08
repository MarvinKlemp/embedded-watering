#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f103xx_hal as bluepill_hal;

use cortex_m_rt::entry;
use bluepill_hal::delay::Delay;
use bluepill_hal::prelude::*;
use bluepill_hal::stm32f103xx::Peripherals;

#[entry]
fn main() -> ! {
    let bluepill = Peripherals::take().unwrap();

    let mut rcc = bluepill.RCC.constrain();
    let mut flash = bluepill.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = bluepill.GPIOC.split(&mut rcc.apb2);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        led.set_high();
        delay.delay_ms(1_000_u16);

        //  Output 0V on the LED Pin and show a message in OpenOCD console.
        led.set_low();
        delay.delay_ms(1_000_u16);
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {

    }
}

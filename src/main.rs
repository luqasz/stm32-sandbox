#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
extern crate panic_halt;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use nb::block;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}

#[cfg_attr(not(test), entry)]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them
    // into the corresponding HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store
    // the frozen frequencies in `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is
    // passed to the function in order to configure the port. For pins 0-7,
    // crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, 4.hz(), clocks);

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        block!(timer.wait()).unwrap();
        led.set_high().unwrap();
        block!(timer.wait()).unwrap();
        led.set_low().unwrap();
    }
}

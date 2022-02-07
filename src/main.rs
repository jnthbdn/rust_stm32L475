#![no_std]
#![no_main]

use cortex_m::Peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::hio;
use panic_semihosting as _;

use stm32l4xx_hal::{stm32, prelude::*, delay::Delay};

use core::fmt::Write;


#[entry]
fn main() -> !{    
    let mut hstdout = hio::hstdout().unwrap();

    writeln!(hstdout, "Hello, world!").unwrap();

    let cortex_peripherals = Peripherals::take().unwrap();
    let device_peripherals = stm32::Peripherals::take().unwrap();

    let mut flash = device_peripherals.FLASH.constrain(); // .constrain();
    let mut rcc = device_peripherals.RCC.constrain();
    let mut pwr = device_peripherals.PWR.constrain(&mut rcc.apb1r1);

    // Try a different clock configuration
    let clocks = rcc.cfgr.hclk(8.mhz()).freeze(&mut flash.acr, &mut pwr);
    // let clocks = rcc.cfgr
    //     .sysclk(64.mhz())
    //     .pclk1(32.mhz())
    //     .freeze(&mut flash.acr);

    // let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);
    // let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.afrh);

    let mut gpiob = device_peripherals.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpiob
        .pb3
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut timer = Delay::new(cortex_peripherals.SYST, clocks);
    loop {
        // block!(timer.wait()).unwrap();
        timer.delay_ms(1000_u32);
        led.set_high().unwrap();
        // block!(timer.wait()).unwrap();
        timer.delay_ms(1000_u32);
        led.set_low().unwrap();
    }
}
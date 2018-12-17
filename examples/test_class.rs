#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f103xx_hal as hal;
extern crate usb_device;
extern crate stm32f103xx_usb;

use hal::prelude::*;
use hal::stm32f103xx;
use rt::ExceptionFrame;

use usb_device::test_class::TestClass;
use stm32f103xx_usb::UsbBus;

entry!(main);
fn main() -> ! {
    let dp = stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .use_hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid(), "usb clocks not valid");

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    let usb_bus = UsbBus::usb_with_reset(
        dp.USB, &mut rcc.apb1,
        &clocks, &mut gpioa.crh, gpioa.pa12);

    let test = TestClass::new(&usb_bus);

    let mut usb_dev = test.make_device(&usb_bus);

    usb_dev.force_reset().expect("reset failed");

    loop {
        usb_dev.poll();
        test.poll();
    }
}

exception!(HardFault, hard_fault);
fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);
fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

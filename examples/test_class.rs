#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32f1xx_hal::{prelude::*, stm32};

use usb_device::test_class::TestClass;
use stm32f103xx_usb::{UsbBus, ResetPin};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .use_hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid(), "usb clocks not valid");

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    let reset_pin = ResetPin::new(gpioa.pa12, &mut gpioa.crh);

    let usb_bus = UsbBus::usb_with_reset(
        dp.USB, &mut rcc.apb1,
        &clocks, reset_pin);

    let mut test = TestClass::new(&usb_bus);

    let mut usb_dev = { test.make_device(&usb_bus) };

    usb_dev.force_reset().expect("reset failed");

    loop {
        if usb_dev.poll(&mut [&mut test]) {
            test.poll();
        }
    }
}

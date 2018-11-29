#![no_std]
#![no_main]

/// Polling based USB serial sample

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

use usb_device::prelude::*;
use stm32f103xx_usb::UsbBus;

mod cdc_acm;

entry!(main);
fn main() -> ! {
    let dp = stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid());

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let usb_bus = UsbBus::usb(dp.USB, &mut rcc.apb1);
    usb_bus.init(|b| b.enable_reset(&clocks, &mut gpioa.crh, gpioa.pa12));

    let custom = example::CustomClass::new();

    let mut usb_dev = UsbDevice::new(&usb_bus, UsbVidPid(0x1337, 0x7331))
        .manufacturer("Fake company")
        .product("My device")
        .build(&[&custom]);

    usb_dev.force_reset().expect("reset failed");

    loop {
        // Could be in an interrupt
        usb_dev.poll();

        if let Some(v) = custom.recv() {
            if v == 0 {
                led.set_high();
            } else {
                led.set_low();
            }
        }
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

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

use usb_device::prelude::*;
use stm32f103xx_usb::UsbBus;

mod example {
    use core::sync::atomic::{AtomicUsize, Ordering};
    use usb_device::class_prelude::*;

    pub struct CustomClass {
        value: AtomicUsize,
    }

    impl CustomClass {
        pub fn new() -> CustomClass {
            CustomClass {
                value: AtomicUsize::new(::core::usize::MAX),
            }
        }

        pub fn recv(&self) -> Option<u8> {
            match self.value.swap(::core::usize::MAX, Ordering::SeqCst) {
                ::core::usize::MAX => None,
                v => Some(v as u8),
            }
        }
    }

    impl UsbClass for CustomClass {
        fn control_in(&self, req: &control::Request, buf: &mut [u8]) -> ControlInResult {
            if req.request_type == control::RequestType::Vendor
                && req.recipient == control::Recipient::Device
                && req.length >= 2
            {
                buf[..2].copy_from_slice(&[0x13, 0x37]);
                ControlInResult::Ok(2)
            } else {
                ControlInResult::Ignore
            }
        }

        fn control_out(&self, req: &control::Request, _buf: &[u8]) -> ControlOutResult {
            if req.request_type == control::RequestType::Vendor
                && req.recipient == control::Recipient::Device
            {
                self.value.store(req.value as usize, Ordering::SeqCst);
                ControlOutResult::Ok
            } else {
                ControlOutResult::Ignore
            }
        }
    }
}

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
    usb_bus.borrow_mut().enable_reset(&clocks, &mut gpioa.crh, gpioa.pa12);

    let custom = example::CustomClass::new();

    let usb_dev = UsbDevice::new(&usb_bus, UsbVidPid(0x1337, 0x7331))
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

#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
#[macro_use]
extern crate stm32f103xx;
extern crate stm32f103xx_hal as hal;
extern crate usb_device;
extern crate stm32f103xx_usb;

use stm32f103xx::Interrupt;
use hal::prelude::*;
use rt::ExceptionFrame;
use cortex_m::asm::wfi;

use usb_device::prelude::*;
use usb_device::bus::UsbBusAllocator;
use stm32f103xx_usb::UsbBus;

mod cdc_acm;

static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_SERIAL: Option<cdc_acm::SerialPort<UsbBus>> = None;
static mut USB_DEVICE: Option<UsbDevice<UsbBus>> = None;

entry!(main);
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .use_hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid());

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    // Unsafe to allow access to static variables
    unsafe {
        let bus = UsbBus::usb_with_reset(dp.USB,
            &mut rcc.apb1, &clocks, &mut gpioa.crh, gpioa.pa12);

        USB_BUS = Some(bus);

        USB_SERIAL = Some(cdc_acm::SerialPort::new(USB_BUS.as_ref().unwrap()));

        let mut usb_dev = UsbDevice::new(
                USB_BUS.as_ref().unwrap(),
                UsbVidPid(0x5824, 0x27dd),
                &[USB_SERIAL.as_ref().unwrap()])
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")
            .device_class(cdc_acm::USB_CLASS_CDC)
            .build();

        usb_dev.force_reset().expect("reset failed");

        USB_DEVICE = Some(usb_dev);
    }

    let mut nvic = p.NVIC;

    nvic.enable(Interrupt::CAN1_TX);
    nvic.enable(Interrupt::CAN1_RX0);

    loop { wfi(); }
}

exception!(HardFault, hard_fault);
fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);
fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

interrupt!(CAN1_TX, usb_interrupt);
interrupt!(CAN1_RX0, usb_interrupt);

fn usb_interrupt() {
    let usb_dev = unsafe { USB_DEVICE.as_mut().unwrap() };
    let serial = unsafe { USB_SERIAL.as_ref().unwrap() };

    if !usb_dev.poll() {
        return;
    }

    let mut buf = [0u8; 8];

    match serial.read(&mut buf) {
        Ok(count) if count > 0 => {
            // Echo back in upper case
            for c in buf[0..count].iter_mut() {
                if 0x61 <= *c && *c <= 0x7a {
                    *c &= !0x20;
                }
            }

            serial.write(&buf[0..count]).ok();
        },
        _ => { },
    }
}
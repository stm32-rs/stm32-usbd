#![no_std]
#![no_main]

/// CDC-ACM serial port example using interrupts.

extern crate panic_semihosting;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use stm32f1xx_hal::{prelude::*, stm32};
use stm32f1xx_hal::stm32::{interrupt, Interrupt};

use usb_device::{prelude::*, bus::UsbBusAllocator};
use stm32_usbd::{UsbBus, ResetPin};

mod cdc_acm;

static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_SERIAL: Option<cdc_acm::SerialPort<UsbBus>> = None;
static mut USB_DEVICE: Option<UsbDevice<UsbBus>> = None;

#[entry]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .use_hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid());

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    let reset_pin = ResetPin::new(gpioa.pa12, &mut gpioa.crh);

    // Unsafe to allow access to static variables
    unsafe {
        let bus = UsbBus::usb_with_reset(dp.USB,
            &mut rcc.apb1, &clocks, reset_pin);

        USB_BUS = Some(bus);

        USB_SERIAL = Some(cdc_acm::SerialPort::new(USB_BUS.as_ref().unwrap()));

        let mut usb_dev = UsbDeviceBuilder::new(
                USB_BUS.as_ref().unwrap(),
                UsbVidPid(0x5824, 0x27dd))
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")
            .device_class(cdc_acm::USB_CLASS_CDC)
            .build();

        usb_dev.force_reset().expect("reset failed");

        USB_DEVICE = Some(usb_dev);
    }

    let mut nvic = p.NVIC;

    nvic.enable(Interrupt::USB_HP_CAN_TX);
    nvic.enable(Interrupt::USB_LP_CAN_RX0);

    loop { wfi(); }
}

#[interrupt]
fn USB_HP_CAN_TX() {
    usb_interrupt();
}

#[interrupt]
fn USB_LP_CAN_RX0() {
    usb_interrupt();
}

fn usb_interrupt() {
    let usb_dev = unsafe { USB_DEVICE.as_mut().unwrap() };
    let serial = unsafe { USB_SERIAL.as_mut().unwrap() };

    if !usb_dev.poll(&mut [serial]) {
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

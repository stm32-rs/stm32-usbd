#![no_std]
#![feature(asm)]

extern crate cortex_m;
extern crate stm32f103xx;
extern crate stm32f103xx_hal;
extern crate vcell;
extern crate usb_device;

mod regs;
pub mod bus;
pub use bus::UsbBus;
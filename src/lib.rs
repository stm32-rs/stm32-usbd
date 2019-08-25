//! USB peripheral driver for STM32 microcontrollers.
//!
//! This also serves as the reference implementation and example repository for the `usb-device`
//! crate for now.

#![no_std]

mod endpoint;
mod endpoint_memory;

mod target;

pub mod bus;

pub use crate::bus::UsbBus;
pub use crate::target::usb_pins::UsbPinsType;
pub type UsbBusType = UsbBus<UsbPinsType>;

mod pac;

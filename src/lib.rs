//! USB peripheral driver for STM32 microcontrollers.
//!
//! This also serves as the reference implementation and example repository for the `usb-device`
//! crate for now.

#![no_std]

mod endpoint;
mod endpoint_memory;

mod atomic_mutex;
mod target;

/// USB peripheral driver.
pub mod bus;

pub use crate::bus::{UsbBus, ResetPin};

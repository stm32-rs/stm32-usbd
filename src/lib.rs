//! USB peripheral driver for STM32 microcontrollers.
//!
//! This also serves as the reference implementation and example repository for the `usb-device`
//! crate for now.

#![no_std]

#[cfg(not(feature = "family-selected"))]
compile_error!("This crate requires one of the device family features enabled.
Check Cargo.toml for supported families ('Device family' section)");

#[cfg(feature = "family-selected")]
mod endpoint;

#[cfg(feature = "family-selected")]
mod endpoint_memory;

#[cfg(feature = "family-selected")]
mod target;

#[cfg(feature = "family-selected")]
pub mod bus;

#[cfg(feature = "family-selected")]
pub use crate::bus::UsbBus;
#[cfg(feature = "family-selected")]
pub use crate::target::usb_pins::UsbPinsType;
#[cfg(feature = "family-selected")]
pub type UsbBusType = UsbBus<UsbPinsType>;

mod pac;

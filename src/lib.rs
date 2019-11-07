//! USB peripheral driver for STM32 microcontrollers.
//!
//! This also serves as the reference implementation and example repository for the `usb-device`
//! crate for now.

#![no_std]

#[cfg(not(feature = "family-selected"))]
compile_error!(
    "This crate requires one of the device family features enabled.
Check Cargo.toml for supported families ('Device family' section)"
);

#[cfg(feature = "family-selected")]
mod endpoint;

#[cfg(feature = "family-selected")]
mod endpoint_memory;

mod registers;

#[cfg(feature = "family-selected")]
mod target;

#[cfg(feature = "family-selected")]
pub mod bus;

#[cfg(feature = "family-selected")]
pub use crate::bus::UsbBus;

mod pac;

/// A trait for device-specific USB peripherals. Implement this to add support for a new hardware
/// platform. Peripherals that have this trait must have the same register block as STM32 USBFS
/// peripherals.
pub unsafe trait UsbPeripheral: Send + Sync {
    /// Pointer to the register block
    const REGISTERS: *const ();
}

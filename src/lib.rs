//! USB peripheral driver for STM32 microcontrollers.
//!
//! This also serves as the reference implementation and example repository for the `usb-device`
//! crate for now.

#![no_std]

pub mod bus;
mod endpoint;
mod endpoint_memory;
mod registers;
pub use crate::bus::UsbBus;

mod pac;

/// A trait for device-specific USB peripherals. Implement this to add support for a new hardware
/// platform. Peripherals that have this trait must have the same register block as STM32 USBFS
/// peripherals.
pub unsafe trait UsbPeripheral: Send + Sync {
    /// Pointer to the register block
    const REGISTERS: *const ();

    /// Embedded pull-up resistor on USB_DP line
    const DP_PULL_UP_FEATURE: bool;

    /// Pointer to the endpoint memory
    const EP_MEMORY: *const ();

    /// Endpoint memory size in bytes
    const EP_MEMORY_SIZE: usize;

    /// Endpoint memory access scheme
    ///
    /// Check Reference Manual for details.
    /// Set to `true` if "2x16 bits/word" access scheme is used, otherwise set to `false`.
    const EP_MEMORY_ACCESS_2X16: bool;

    type Word: Word;

    /// Enables USB device on its peripheral bus
    fn enable();

    /// Performs a chip specific startup delay
    ///
    /// This function is called in `UsbBus::enable()` after deasserting the `pdwn` bit and before
    /// peripheral initialization.
    fn startup_delay();
}

pub trait Word: From<u16> + Into<u32> + Copy + Send + 'static
{
    fn from_iter_le<'a>(it: &mut impl Iterator<Item = &'a u8>) -> Self;
    fn write_to_iter_le<'a>(self, it: &mut impl Iterator<Item = &'a mut u8>);
}

impl Word for u16 {
    fn from_iter_le<'a>(it: &mut impl Iterator<Item = &'a u8>) -> Self {
        Self::from_le_bytes([
            *it.next().unwrap_or(&0),
            *it.next().unwrap_or(&0),
        ])
    }

    fn write_to_iter_le<'a>(self, it: &mut impl Iterator<Item = &'a mut u8>) {
        for (w, r) in it.zip(self.to_le_bytes()) {
            *w = r;
        }
    }
}

impl Word for u32 {
    fn from_iter_le<'a>(it: &mut impl Iterator<Item = &'a u8>) -> Self {
        Self::from_le_bytes([
            *it.next().unwrap_or(&0),
            *it.next().unwrap_or(&0),
            *it.next().unwrap_or(&0),
            *it.next().unwrap_or(&0),
        ])
    }

    fn write_to_iter_le<'a>(self, it: &mut impl Iterator<Item = &'a mut u8>) {
        for (w, r) in it.zip(self.to_le_bytes()) {
            *w = r;
        }
    }
}
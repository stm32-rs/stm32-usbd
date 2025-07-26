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
pub use crate::endpoint_memory::MemoryAccess;

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
    /// See `MemoryAccess` enum for more details. Check Reference Manual to determine the correct access scheme to use.
    const EP_MEMORY_ACCESS: MemoryAccess;

    /// Enables USB device on its peripheral bus
    fn enable();

    /// Performs a chip specific startup delay
    ///
    /// This function is called in `UsbBus::enable()` after deasserting the `pdwn` bit and before
    /// peripheral initialization.
    fn startup_delay();
}

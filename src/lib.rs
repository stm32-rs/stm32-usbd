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
    /// Can be any of:
    /// * U16Bits
    /// * U16BitsX2
    /// * U32Bits
    type SramAccessScheme: SramAccessScheme;

    /// Enables USB device on its peripheral bus
    fn enable();

    /// Performs a chip specific startup delay
    ///
    /// This function is called in `UsbBus::enable()` after deasserting the `pdwn` bit and before
    /// peripheral initialization.
    fn startup_delay();
}

pub struct U16Bits;
pub struct U16BitsX2;
pub struct U32Bits;

pub enum AccessType {
    Tx = 0,
    Rx = 1,
}

pub trait SramAccessScheme {
    type Word: Word;
    const ADDRESS_MULTIPLIER: usize = 1;

    unsafe fn set(ptr: *mut Self::Word, t: AccessType, address: u16, count: u16);
    unsafe fn read(ptr: *const Self::Word, t: AccessType) -> (u16, u16);
}

impl SramAccessScheme for U16Bits {
    type Word = u16;
    const ADDRESS_MULTIPLIER: usize = 2;

    unsafe fn set(ptr: *mut Self::Word, t: AccessType, address: u16, count: u16) {
        set(ptr, t, Self::ADDRESS_MULTIPLIER, address, count);
    }

    unsafe fn read(ptr: *const Self::Word, t: AccessType) -> (u16, u16) {
        read(ptr, t, Self::ADDRESS_MULTIPLIER)
    }
}

impl SramAccessScheme for U16BitsX2 {
    type Word = u16;

    unsafe fn set(ptr: *mut Self::Word, t: AccessType, address: u16, count: u16) {
        set(ptr, t, Self::ADDRESS_MULTIPLIER, address, count);
    }

    unsafe fn read(ptr: *const Self::Word, t: AccessType) -> (u16, u16) {
        read(ptr, t, Self::ADDRESS_MULTIPLIER)
    }
}
impl SramAccessScheme for U32Bits {
    type Word = u32;

    unsafe fn set(ptr: *mut Self::Word, t: AccessType, address: u16, count: u16) {
        assert!(count < (1 << 10));

        let offset = t as usize;

        let bits = u32::from(count) << 16 | u32::from(address);
        unsafe {
            ptr.add(offset).write_volatile(bits);
        }
    }

    unsafe fn read(ptr: *const Self::Word, t: AccessType) -> (u16, u16) {
        let offset = t as usize;

        unsafe {
            let bits = ptr.add(offset).read_volatile();
            let address = bits as u16;
            let count = (bits >> 16) as u16;

            (address, count)
        }
    }
}

pub trait Word: From<u16> + Into<u32> + Copy + Send + 'static {
    fn from_iter_le<'a>(it: &mut impl Iterator<Item = &'a u8>) -> Self;
    fn write_to_iter_le<'a>(self, it: &mut impl Iterator<Item = &'a mut u8>);
}

impl Word for u16 {
    fn from_iter_le<'a>(it: &mut impl Iterator<Item = &'a u8>) -> Self {
        Self::from_le_bytes([*it.next().unwrap_or(&0), *it.next().unwrap_or(&0)])
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

fn set(ptr: *mut u16, t: AccessType, mul: usize, address: u16, count: u16) {
    assert!(count < (1 << 10));

    let offset = t as usize;

    unsafe {
        ptr.add(offset * mul).write_volatile(address);
        ptr.add((offset + 1) * mul).write_volatile(count);
    }
}

fn read(ptr: *const u16, t: AccessType, mul: usize) -> (u16, u16) {
    let offset = t as usize;

    unsafe {
        let address = ptr.add(offset * mul).read_volatile();
        let count = ptr.add((offset + 1) * mul).read_volatile();

        (address, count)
    }
}

//! Target-specific definitions

// Export HAL
#[cfg(feature = "stm32f042xx")]
pub use stm32f0xx_hal as hal;
#[cfg(feature = "stm32f103xx")]
pub use stm32f1xx_hal as hal;
#[cfg(feature = "stm32l4x2xx")]
pub use stm32l4xx_hal as hal;


// USB PAC reexports
#[cfg(feature = "stm32f042xx")]
pub use hal::stm32::USB;
#[cfg(feature = "stm32f103xx")]
pub use hal::stm32::USB;
#[cfg(feature = "stm32l4x2xx")]
pub use hal::stm32::USB;

// Use bundled register definitions instead of device-specific ones
// This should work because register definitions from newer chips seem to be
// compatible with definitions for older ones.
pub use crate::pac::usb;


#[cfg(usb_access_scheme = "1x16")]
pub type UsbAccessType = u32;
#[cfg(usb_access_scheme = "2x16")]
pub type UsbAccessType = u16;


#[cfg(any(feature = "stm32f103xx", feature = "stm32f042xx"))]
pub const EP_MEM_ADDR: usize = 0x4000_6000;
#[cfg(feature = "stm32l4x2xx")]
pub const EP_MEM_ADDR: usize = 0x4000_6C00;


#[cfg(usb_buffer_size = "512")]
pub const EP_MEM_SIZE: usize = 512;
#[cfg(usb_buffer_size = "1024")]
pub const EP_MEM_SIZE: usize = 1024;


pub const NUM_ENDPOINTS: usize = 8;


/// Enables USB peripheral
pub fn apb_usb_enable() {
    cortex_m::interrupt::free(|_| {
        let rcc = unsafe { (&*hal::stm32::RCC::ptr()) };
        match () {
            #[cfg(any(feature = "stm32f103xx", feature = "stm32f042xx"))]
            () => rcc.apb1enr.modify(|_, w| w.usben().set_bit()),
            #[cfg(feature = "stm32l4x2xx")]
            () => rcc.apb1enr1.modify(|_, w| w.usbfsen().set_bit()),
        }
    });
}


use hal::prelude::*;
use hal::gpio::{self, gpioa};

/// Device-dependent wrapper for USB D+ pin
pub struct ResetPin {
    pin: gpioa::PA12<gpio::Output<gpio::PushPull>>,
}

impl ResetPin {
    #[cfg(feature = "stm32f103xx")]
    pub fn new<M>(pa12: gpioa::PA12<M>, crh: &mut gpioa::CRH) -> Self {
        Self {
            pin: pa12.into_push_pull_output(crh)
        }
    }

    #[cfg(feature = "stm32l4x2xx")]
    pub fn new<M>(pa12: gpioa::PA12<M>, moder: &mut gpioa::MODER, otyper: &mut gpioa::OTYPER) -> Self {
        Self {
            pin: pa12.into_push_pull_output(moder, otyper)
        }
    }

    pub fn set_low(&mut self) {
        self.pin.set_low()
    }
}

/// Wrapper around device-specific peripheral that provides unified register interface
pub struct UsbRegisters(USB);

impl core::ops::Deref for UsbRegisters {
    type Target = usb::RegisterBlock;

    fn deref(&self) -> &Self::Target {
        let ptr = USB::ptr() as *const Self::Target;
        unsafe { &*ptr }
    }
}

impl UsbRegisters {
    pub fn new(usb: USB) -> Self {
        Self(usb)
    }

    pub fn ep_register(index: u8) -> &'static usb::EPR {
        let usb_ptr = USB::ptr() as *const usb::RegisterBlock;
        unsafe { &(*usb_ptr).epr[index as usize] }
    }
}

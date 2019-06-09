//! Target-specific definitions

// Export HAL
#[cfg(feature = "stm32f0xx-hal")]
pub use stm32f0xx_hal as hal;
#[cfg(feature = "stm32f103xx")]
pub use stm32f1xx_hal as hal;
#[cfg(feature = "stm32f303xc")]
pub use stm32f3xx_hal as hal;
#[cfg(feature = "stm32l4x2xx")]
pub use stm32l4xx_hal as hal;


// USB PAC reexports
#[cfg(feature = "stm32f0xx-hal")]
pub use hal::stm32::USB;
#[cfg(feature = "stm32f103xx")]
pub use hal::stm32::USB;
#[cfg(feature = "stm32f303xc")]
pub use hal::stm32::USB_FS as USB;
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


#[cfg(any(feature = "stm32f103xx", feature = "stm32f0xx-hal", feature = "stm32f303xc"))]
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
            #[cfg(any(feature = "stm32f103xx", feature = "stm32f0xx-hal", feature = "stm32f303xc"))]
            () => rcc.apb1enr.modify(|_, w| w.usben().set_bit()),
            #[cfg(feature = "stm32l4x2xx")]
            () => rcc.apb1enr1.modify(|_, w| w.usbfsen().set_bit()),
        }
    });
}

// Workaround: cortex-m contains pre-compiled __delay function
// on which rust-lld fails with "unrecognized reloc 103"
// https://github.com/rust-embedded/cortex-m/issues/125
#[cfg(usb_delay_workaround)]
pub fn delay(n: u32) {
    #[inline(never)]
    fn __delay(mut n: u32) {
        while n > 0 {
            n -= 1;
            cortex_m::asm::nop();
        }
    }

    __delay(n / 4 + 1);
}
#[cfg(not(usb_delay_workaround))]
pub use cortex_m::asm::delay;


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



pub trait UsbPins: Send { }

#[cfg(feature = "stm32f0xx-hal")]
pub mod usb_pins {
    use super::hal::gpio::{Input, Floating};
    use super::hal::gpio::gpioa::{PA11, PA12};

    pub type UsbPinsType = (PA11<Input<Floating>>, PA12<Input<Floating>>);
    impl super::UsbPins for UsbPinsType {}
}

#[cfg(feature = "stm32f103xx")]
pub mod usb_pins {
    use super::hal::gpio::{Input, Floating};
    use super::hal::gpio::gpioa::{PA11, PA12};

    pub type UsbPinsType = (PA11<Input<Floating>>, PA12<Input<Floating>>);
    impl super::UsbPins for UsbPinsType {}
}

#[cfg(feature = "stm32f303xc")]
pub mod usb_pins {
    use super::hal::gpio::AF14;
    use super::hal::gpio::gpioa::{PA11, PA12};

    pub type UsbPinsType = (PA11<AF14>, PA12<AF14>);
    impl super::UsbPins for UsbPinsType {}
}

#[cfg(feature = "stm32l4x2xx")]
pub mod usb_pins {
    use super::hal::gpio::{AF10, Alternate, Input, Floating};
    use super::hal::gpio::gpioa::{PA11, PA12};

    pub type UsbPinsType = (
        PA11<Alternate<AF10, Input<Floating>>>,
        PA12<Alternate<AF10, Input<Floating>>>
    );
    impl super::UsbPins for UsbPinsType {}
}

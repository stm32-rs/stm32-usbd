//! Target-specific definitions

// Export HAL
#[cfg(feature = "stm32f103xx")]
pub use stm32f1xx_hal as hal;
#[cfg(feature = "stm32l4x2xx")]
pub use stm32l4xx_hal as hal;


// USB PAC reexports
#[cfg(feature = "stm32f103xx")]
pub use hal::stm32::usb;
use cortex_m::interrupt;
#[cfg(feature = "stm32l4x2xx")]
pub mod usb {
    pub use super::hal::stm32::usb::EP0R as EPR;
    pub use super::hal::stm32::usb::ep0r as epr;
}


/// Returns EP register reference by its index
pub fn ep_reg(index: u8) -> &'static usb::EPR {
    unsafe {
        let usb = &(*hal::stm32::USB::ptr());
        match () {
            #[cfg(feature = "stm32f103xx")]
            () => &usb.epr[index as usize],
            #[cfg(feature = "stm32l4x2xx")]
            () => {
                let ep0r_ptr: *const usb::EPR = &usb.ep0r;
                &*ep0r_ptr.offset(index as isize)
            },
        }
    }
}


// Device-specific bus definition
#[cfg(feature = "stm32f103xx")]
pub type APB = hal::rcc::APB1;
#[cfg(feature = "stm32l4x2xx")]
pub type APB = hal::rcc::APB1R1;


/// Enables USB peripheral
pub fn apb_usb_enable(apb: &mut APB) {
    let _ = apb;
    interrupt::free(|_| {
        let rcc = unsafe { (&*hal::stm32::RCC::ptr()) };
        match () {
            #[cfg(feature = "stm32f103xx")]
            () => rcc.apb1enr.modify(|_, w| w.usben().set_bit()),
            #[cfg(feature = "stm32l4x2xx")]
            () => rcc.apb1enr1.modify(|_, w| w.usbfsen().set_bit()),
        }
    });
}


use hal::prelude::*;
use hal::gpio::{self, gpioa};

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

//! USB peripheral

use stm32_usbd::{MemoryAccess, UsbPeripheral};
use stm32f1xx_hal::pac::{RCC, USB};

pub use stm32_usbd::UsbBus;
use stm32f1xx_hal::gpio::gpioa::{PA11, PA12};
use stm32f1xx_hal::gpio::{Floating, Input};

pub struct Peripheral {
    pub usb: USB,
    pub pin_dm: PA11<Input<Floating>>,
    pub pin_dp: PA12<Input<Floating>>,
}

unsafe impl Sync for Peripheral {}

unsafe impl UsbPeripheral for Peripheral {
    const REGISTERS: *const () = USB::ptr() as *const ();
    const DP_PULL_UP_FEATURE: bool = false;
    const EP_MEMORY: *const () = 0x4000_6000 as _;
    const EP_MEMORY_SIZE: usize = 512;
    const EP_MEMORY_ACCESS: MemoryAccess = MemoryAccess::Word16x1;

    fn enable() {
        let rcc = unsafe { &*RCC::ptr() };

        cortex_m::interrupt::free(|_| {
            // Enable USB peripheral
            rcc.apb1enr.modify(|_, w| w.usben().set_bit());

            // Reset USB peripheral
            rcc.apb1rstr.modify(|_, w| w.usbrst().set_bit());
            rcc.apb1rstr.modify(|_, w| w.usbrst().clear_bit());
        });
    }

    fn startup_delay() {
        // There is a chip specific startup delay. For STM32F103xx it's 1µs and this should wait for
        // at least that long.
        cortex_m::asm::delay(72);
    }
}

pub type UsbBusType = UsbBus<Peripheral>;

fn main() -> ! {
    loop {}
}

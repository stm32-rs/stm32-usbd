use crate::pac::usb::{RegisterBlock, EPR};
use crate::UsbPeripheral;
use core::marker::PhantomData;

/// A proxy type that provides unified register interface
pub struct UsbRegisters<USB> {
    _marker: PhantomData<USB>,
}

impl<USB: UsbPeripheral> core::ops::Deref for UsbRegisters<USB> {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        let ptr = USB::REGISTERS as *const Self::Target;
        unsafe { &*ptr }
    }
}

impl<USB: UsbPeripheral> UsbRegisters<USB> {
    pub fn new() -> Self {
        Self { _marker: PhantomData }
    }

    pub fn ep_register(index: u8) -> &'static EPR {
        let usb_ptr = USB::REGISTERS as *const RegisterBlock;
        unsafe { &(*usb_ptr).epr[index as usize] }
    }
}

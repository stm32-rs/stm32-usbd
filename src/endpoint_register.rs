use core::mem;
use cortex_m::interrupt;
use usb_device::endpoint::EndpointType;
use crate::bus::UsbBus;
use crate::target::usb;

/// Wrapper for the endpoint-specific registers that handles race conditions between hardware as
/// well as software.
pub struct EndpointRegister(u8);

impl EndpointRegister {
    pub fn get(number: u8) -> EndpointRegister {
        EndpointRegister(number)
    }

    fn reg(&self) -> &'static mut usb::EPR {
        &mut UsbBus::regs().epr[self.0 as usize]
    }

    pub fn read(&self) -> usb::epr::R {
        self.reg().read()
    }

    /// The endpoint register fields may be modified by hardware as well as software. To avoid race
    /// conditions, there are invariant values for the fields that may be modified by the hardware
    /// that can be written to avoid modifying other fields while modifying a single field. This
    /// method sets all the volatile fields to their invariant values.
    fn modify<F>(&mut self, f: F)
    where
        for<'w> F: FnOnce(&usb::epr::R, &'w mut usb::epr::W) -> &'w mut usb::epr::W
    {
        interrupt::free(|_| {
            self.reg().modify(|r, w|
                 f(r, w
                    .ctr_rx().set_bit()
                    .dtog_rx().clear_bit()
                    .stat_rx().bits(0)
                    .ctr_tx().set_bit()
                    .dtog_tx().clear_bit()
                    .stat_tx().bits(0)))
        });
    }

    pub fn configure(&mut self, ep_type: EndpointType) {
        let ea = self.0;

        self.modify(|_, w| w
            .ep_type().bits(ep_type.bits())
            .ep_kind().clear_bit()
            .ea().bits(ea));
    }

    pub fn clear_ctr_rx(&mut self) {
        self.modify(|_, w| w.ctr_rx().clear_bit());
    }

    pub fn clear_ctr_tx(&mut self) {
        self.modify(|_, w| w.ctr_tx().clear_bit());
    }

    pub fn set_stat_rx(&mut self, status: EndpointStatus) {
        self.modify(|r, w| w.stat_rx().bits(r.stat_rx().bits() ^ (status as u8)));
    }

    pub fn set_stat_tx(&mut self, status: EndpointStatus) {
        self.modify(|r, w| w.stat_tx().bits(r.stat_tx().bits() ^ (status as u8)));
    }
}


trait EndpointTypeExt {
    fn bits(self) -> u8;
}

impl EndpointTypeExt for EndpointType {
    fn bits(self) -> u8 {
        const BITS: [u8; 4] = [0b01, 0b10, 0b00, 0b11];
        return BITS[self as usize];
    }
}

#[repr(u8)]
#[derive(PartialEq, Eq, Debug)]
#[allow(unused)]
pub enum EndpointStatus {
    Disabled = 0b00,
    Stall = 0b01,
    Nak = 0b10,
    Valid = 0b11,
}

impl From<u8> for EndpointStatus {
    fn from(v: u8) -> EndpointStatus {
        if v <= 0b11 {
            unsafe { mem::transmute(v) }
        } else {
            EndpointStatus::Disabled
        }
    }
}
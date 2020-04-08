use crate::endpoint_memory::{BufferDescriptor, EndpointBuffer, EndpointMemoryAllocator, UsbAccessType};
use crate::registers::UsbRegisters;
use crate::UsbPeripheral;
use core::marker::PhantomData;
use core::mem;
use usb_device::endpoint::{EndpointAddress, EndpointConfig, EndpointType, OutPacketType};
use usb_device::usbcore;
use usb_device::{Result, UsbError, UsbDirection};

// Use bundled register definitions instead of device-specific ones
// This should work because register definitions from newer chips seem to be
// compatible with definitions for older ones.
pub use crate::pac::usb;

pub const NUM_ENDPOINTS: usize = 8;

pub fn calculate_count_rx(mut size: usize) -> Result<(usize, u16)> {
    if size <= 62 {
        // Buffer size is in units of 2 bytes, 0 = 0 bytes
        size = (size + 1) & !0x01;

        let size_bits = size >> 1;

        Ok((size, (size_bits << 10) as u16))
    } else if size <= 1024 {
        // Buffer size is in units of 32 bytes, 0 = 32 bytes
        size = (size + 31) & !0x1f;

        let size_bits = (size >> 5) - 1;

        Ok((size, (0x8000 | (size_bits << 10)) as u16))
    } else {
        Err(UsbError::EndpointMemoryOverflow)
    }
}

/// The endpoint register fields may be modified by hardware as well as software. To avoid race
/// conditions, there are invariant values for the fields that may be modified by the hardware
/// that can be written to avoid modifying other fields while modifying a single field. This
/// method sets all the volatile fields to their invariant values.
fn set_invariant_values(w: &mut usb::epr::W) -> &mut usb::epr::W {
    w.ctr_rx().set_bit();
    w.dtog_rx().clear_bit();
    w.stat_rx().bits(0);
    w.ctr_tx().set_bit();
    w.dtog_tx().clear_bit();
    w.stat_tx().bits(0)
}

/// On this driver endpoints come in bizarrely linked pairs of OUT/IN
pub(crate) struct EndpointPair<USB: UsbPeripheral> {
    pub index: u8,
    _marker: PhantomData<USB>,
}

impl<USB: UsbPeripheral> EndpointPair<USB> {
    pub fn new(index: u8) -> Self {
        Self {
            index,
            _marker: PhantomData,
        }
    }

    pub(crate) fn descr(&self) -> &'static BufferDescriptor {
        EndpointMemoryAllocator::<USB>::buffer_descriptor(self.index)
    }

    pub(crate) fn reg(&self) -> &'static usb::EPR {
        UsbRegisters::<USB>::ep_register(self.index)
    }

    fn set_stat_rx(&self, status: EndpointStatus) {
        self.reg().modify(|r, w| {
            set_invariant_values(w)
                .stat_rx()
                .bits(r.stat_rx().bits() ^ (status as u8))
        });
    }

    fn set_stat_tx(&self, status: EndpointStatus) {
        self.reg().modify(|r, w| {
            set_invariant_values(w)
                .stat_tx()
                .bits(r.stat_tx().bits() ^ (status as u8))
        });
    }

    pub fn clear_ctr_tx(&self) {
        self.reg()
            .modify(|_, w| set_invariant_values(w).ctr_tx().clear_bit());
    }

    pub fn disable_out(&self) {
        self.set_stat_rx(EndpointStatus::Disabled);
    }

    pub fn set_out_stalled(&self, stalled: bool) {
        self.set_stat_rx(if stalled { EndpointStatus::Stall } else { EndpointStatus::Valid })
    }

    pub fn is_out_stalled(&self) -> bool {
        return self.reg().read().stat_rx().is_stall();
    }

    pub fn disable_in(&self) {
        self.set_stat_tx(EndpointStatus::Disabled);
    }

    pub fn set_in_stalled(&self, stalled: bool) {
        self.set_stat_tx(if stalled { EndpointStatus::Stall } else { EndpointStatus::Nak })
    }

    pub fn is_in_stalled(&self) -> bool {
        return self.reg().read().stat_tx().is_stall();
    }
}

pub struct UsbEndpointOut<USB: UsbPeripheral> {
    pair: EndpointPair<USB>,
    buf_size_bytes: u16,
}

impl<USB: UsbPeripheral> UsbEndpointOut<USB> {
    pub(crate) fn new(index: u8, buf_size_bytes: u16) -> Self {
        Self {
            pair: EndpointPair::new(index),
            buf_size_bytes,
        }
    }

    fn buf(&mut self) -> EndpointBuffer {
        let addr = self.pair.descr().addr_rx.get() as usize;

        EndpointBuffer::new::<USB>(addr, self.buf_size_bytes as usize)
    }
}

impl<USB: UsbPeripheral> usbcore::UsbEndpoint for UsbEndpointOut<USB> {
    fn address(&self) -> EndpointAddress {
        EndpointAddress::from_parts(self.pair.index, UsbDirection::Out)
    }

    unsafe fn enable(&mut self, config: &EndpointConfig) {
        self.pair.reg().modify(|_, w|
            set_invariant_values(w)
                .ep_type().bits(config.ep_type().bits())
                .ep_kind().clear_bit()
                .ctr_rx().clear_bit()
                .ea().bits(self.pair.index));

        self.pair.set_stat_rx(EndpointStatus::Valid);
    }

    fn disable(&mut self) {
        self.pair.disable_out();
    }

    fn is_stalled(&self) -> bool {
        self.pair.is_out_stalled()
    }

    fn set_stalled(&mut self, stalled: bool) {
        self.pair.set_out_stalled(stalled);
    }
}

impl<USB: UsbPeripheral> usbcore::UsbEndpointOut for UsbEndpointOut<USB> {
    fn read_packet(&mut self, data: &mut [u8]) -> Result<(usize, OutPacketType)> {
        let reg_v = self.pair.reg().read();

        let status: EndpointStatus = reg_v.stat_rx().bits().into();

        if status == EndpointStatus::Disabled || !reg_v.ctr_rx().bit_is_set() {
            return Err(UsbError::WouldBlock);
        }

        let packet_type = if reg_v.setup().bit_is_set() {
            OutPacketType::Setup
        } else {
            OutPacketType::Data
        };

        self.pair.reg().modify(|_, w| set_invariant_values(w).ctr_rx().clear_bit());

        let count = (self.pair.descr().count_rx.get() & 0x3ff) as usize;
        if count > data.len() {
            return Err(UsbError::BufferOverflow);
        }

        self.buf().read(&mut data[0..count]);

        self.pair.set_stat_rx(EndpointStatus::Valid);

        Ok((count, packet_type))
    }
}

pub struct UsbEndpointIn<USB: UsbPeripheral> {
    pair: EndpointPair<USB>,
    buf_size_bytes: u16,
}

impl<USB: UsbPeripheral> UsbEndpointIn<USB> {
    pub(crate) fn new(index: u8, buf_size_bytes: u16) -> Self {
        Self {
            pair: EndpointPair::new(index),
            buf_size_bytes,
        }
    }

    fn buf(&mut self) -> EndpointBuffer {
        let addr = self.pair.descr().addr_tx.get() as usize;

        EndpointBuffer::new::<USB>(addr, self.buf_size_bytes as usize)
    }
}

impl<USB: UsbPeripheral> usbcore::UsbEndpoint for UsbEndpointIn<USB> {
    fn address(&self) -> EndpointAddress {
        EndpointAddress::from_parts(self.pair.index, UsbDirection::In)
    }

    unsafe fn enable(&mut self, config: &EndpointConfig) {
        self.pair.reg().modify(|_, w|
            set_invariant_values(w)
                .ep_type().bits(config.ep_type().bits())
                .ep_kind().clear_bit()
                .ctr_tx().clear_bit()
                .ea().bits(self.pair.index));

        self.pair.set_stat_tx(EndpointStatus::Nak);
    }

    fn disable(&mut self) {
        self.pair.disable_in();
    }

    fn is_stalled(&self) -> bool {
        self.pair.is_in_stalled()
    }

    fn set_stalled(&mut self, stalled: bool) {
        self.pair.set_in_stalled(stalled);
    }
}

impl<USB: UsbPeripheral> usbcore::UsbEndpointIn for UsbEndpointIn<USB> {
    fn write_packet(&mut self, data: &[u8]) -> Result<()> {
        if data.len() > self.buf().capacity() {
            return Err(UsbError::BufferOverflow);
        }

        let reg = self.pair.reg();

        match reg.read().stat_tx().bits().into() {
            EndpointStatus::Valid | EndpointStatus::Disabled => return Err(UsbError::WouldBlock),
            _ => {}
        };

        self.buf().write(data);
        self.pair.descr().count_tx.set(data.len() as u16 as UsbAccessType);

        self.pair.set_stat_tx(EndpointStatus::Valid);

        Ok(())
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

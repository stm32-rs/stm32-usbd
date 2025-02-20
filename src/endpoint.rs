use crate::endpoint_memory::{BufferDescriptor, EndpointBuffer, EndpointMemoryAllocator};
use crate::registers::UsbRegisters;
use crate::UsbPeripheral;
use core::marker::PhantomData;
use core::mem;
use cortex_m::interrupt::{self, CriticalSection, Mutex};
use usb_device::endpoint::EndpointType;
use usb_device::{Result, UsbError};

// Use bundled register definitions instead of device-specific ones
// This should work because register definitions from newer chips seem to be
// compatible with definitions for older ones.
pub use crate::pac::usb;

pub const NUM_ENDPOINTS: usize = 8;

/// Arbitrates access to the endpoint-specific registers and packet buffer memory.
#[derive(Default)]
pub struct Endpoint<USB: UsbPeripheral> {
    out_buf: Option<Mutex<EndpointBuffer<USB>>>,
    in_buf: Option<Mutex<EndpointBuffer<USB>>>,
    ep_type: Option<EndpointType>,
    index: u8,
    _marker: PhantomData<USB>,
}

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

impl<USB: UsbPeripheral> Endpoint<USB> {
    pub fn new(index: u8) -> Self {
        Self {
            out_buf: None,
            in_buf: None,
            ep_type: None,
            index,
            _marker: PhantomData,
        }
    }

    pub fn ep_type(&self) -> Option<EndpointType> {
        self.ep_type
    }

    pub fn set_ep_type(&mut self, ep_type: EndpointType) {
        self.ep_type = Some(ep_type);
    }

    pub fn is_out_buf_set(&self) -> bool {
        self.out_buf.is_some()
    }

    pub fn set_out_buf(&mut self, buffer: EndpointBuffer<USB>, size_bits: u16) {
        let offset = buffer.offset();
        self.out_buf = Some(Mutex::new(buffer));

        let descr = self.descr();
        descr.addr_rx().set(offset.into());
        descr.count_rx().set(size_bits.into());
    }

    pub fn is_in_buf_set(&self) -> bool {
        self.in_buf.is_some()
    }

    pub fn set_in_buf(&mut self, buffer: EndpointBuffer<USB>) {
        let offset = buffer.offset();
        self.in_buf = Some(Mutex::new(buffer));

        let descr = self.descr();
        descr.addr_tx().set(offset.into());
        descr.count_tx().set(0.into());
    }

    fn descr(&self) -> BufferDescriptor<USB> {
        EndpointMemoryAllocator::<USB>::buffer_descriptor(self.index)
    }

    fn reg(&self) -> &'static usb::EPR {
        UsbRegisters::<USB>::ep_register(self.index)
    }

    pub fn configure(&self, cs: &CriticalSection) {
        let ep_type = match self.ep_type {
            Some(t) => t,
            None => return,
        };

        self.reg().modify(|_, w| {
            Self::set_invariant_values(w);
            w.ctr_rx().clear_bit();
            // dtog_rx
            // stat_rx
            w.ep_type().bits(ep_type.bits());
            w.ep_kind().clear_bit();
            w.ctr_tx().clear_bit();
            // dtog_tx
            // stat_tx
            w.ea().bits(self.index)
        });

        self.set_stat_rx(
            cs,
            if self.out_buf.is_some() {
                EndpointStatus::Valid
            } else {
                EndpointStatus::Disabled
            },
        );

        self.set_stat_tx(
            cs,
            if self.in_buf.is_some() {
                EndpointStatus::Nak
            } else {
                EndpointStatus::Disabled
            },
        );
    }

    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        interrupt::free(|cs| {
            let in_buf = self.in_buf.as_ref().unwrap().borrow(cs);

            if buf.len() > in_buf.capacity() {
                return Err(UsbError::BufferOverflow);
            }

            let reg = self.reg();

            match reg.read().stat_tx().bits().into() {
                EndpointStatus::Valid | EndpointStatus::Disabled => return Err(UsbError::WouldBlock),
                _ => {}
            };

            in_buf.write(buf);
            self.descr().count_tx().set((buf.len() as u16).into());

            self.set_stat_tx(cs, EndpointStatus::Valid);

            Ok(buf.len())
        })
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        interrupt::free(|cs| {
            let out_buf = self.out_buf.as_ref().unwrap().borrow(cs);

            let reg_v = self.reg().read();

            let status: EndpointStatus = reg_v.stat_rx().bits().into();

            if status == EndpointStatus::Disabled || !reg_v.ctr_rx().bit_is_set() {
                return Err(UsbError::WouldBlock);
            }

            self.clear_ctr_rx(cs);

            let count = (self.descr().count_rx().get().into() & 0x3ff) as usize;
            if count > buf.len() {
                return Err(UsbError::BufferOverflow);
            }

            out_buf.read(&mut buf[0..count]);

            self.set_stat_rx(cs, EndpointStatus::Valid);

            Ok(count)
        })
    }

    pub fn read_reg(&self) -> usb::epr::R {
        self.reg().read()
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

    pub fn clear_ctr_rx(&self, _cs: &CriticalSection) {
        self.reg()
            .modify(|_, w| Self::set_invariant_values(w).ctr_rx().clear_bit());
    }

    pub fn clear_ctr_tx(&self, _cs: &CriticalSection) {
        self.reg()
            .modify(|_, w| Self::set_invariant_values(w).ctr_tx().clear_bit());
    }

    pub fn set_stat_rx(&self, _cs: &CriticalSection, status: EndpointStatus) {
        self.reg().modify(|r, w| {
            Self::set_invariant_values(w)
                .stat_rx()
                .bits(r.stat_rx().bits() ^ (status as u8))
        });
    }

    pub fn set_stat_tx(&self, _cs: &CriticalSection, status: EndpointStatus) {
        self.reg().modify(|r, w| {
            Self::set_invariant_values(w)
                .stat_tx()
                .bits(r.stat_tx().bits() ^ (status as u8))
        });
    }
}

trait EndpointTypeExt {
    fn bits(self) -> u8;
}

impl EndpointTypeExt for EndpointType {
    fn bits(self) -> u8 {
        const BITS: [u8; 4] = [0b01, 0b10, 0b00, 0b11];
        let transfer_type = self.to_bm_attributes() & 0b11;
        return BITS[transfer_type as usize];
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

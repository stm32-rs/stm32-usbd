use core::mem;
use bare_metal::CriticalSection;
use cortex_m::interrupt;
use usb_device::{Result, UsbError};
use usb_device::endpoint::EndpointType;
use crate::atomic_mutex::AtomicMutex;
use crate::target::{usb, ep_reg, UsbAccessType};
use crate::endpoint_memory::{EndpointBuffer, BufferDescriptor, EndpointMemoryAllocator};


/// Arbitrates access to the endpoint-specific registers and packet buffer memory.
#[derive(Default)]
pub struct Endpoint {
    out_buf: Option<AtomicMutex<EndpointBuffer>>,
    in_buf: Option<AtomicMutex<EndpointBuffer>>,
    ep_type: Option<EndpointType>,
    index: u8,
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

impl Endpoint {
    pub fn new(index: u8) -> Endpoint {
        Endpoint {
            out_buf: None,
            in_buf: None,
            ep_type: None,
            index,
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

    pub fn set_out_buf(&mut self, buffer: EndpointBuffer, size_bits: u16) {
        let offset = buffer.offset();
        self.out_buf = Some(AtomicMutex::new(buffer));

        let descr = self.descr();
        descr.addr_rx.set(offset as UsbAccessType);
        descr.count_rx.set(size_bits as UsbAccessType);
    }

    pub fn is_in_buf_set(&self) -> bool {
        self.in_buf.is_some()
    }

    pub fn set_in_buf(&mut self, buffer: EndpointBuffer) {
        let offset = buffer.offset();
        self.in_buf = Some(AtomicMutex::new(buffer));

        let descr = self.descr();
        descr.addr_tx.set(offset as UsbAccessType);
        descr.count_tx.set(0);
    }

    fn descr(&self) -> &'static BufferDescriptor {
        EndpointMemoryAllocator::buffer_descriptor(self.index)
    }

    fn reg(&self) -> &'static usb::EPR {
        ep_reg(self.index)
    }

    pub fn configure(&self, cs: &CriticalSection) {
        let ep_type = match self.ep_type {
            Some(t) => t,
            None => { return },
        };

        self.reg().modify(|_, w| unsafe {
            Self::clear_toggle_bits(w)
                .ctr_rx().clear_bit()
                // dtog_rx
                // stat_rx
                .ep_type().bits(ep_type.bits())
                .ep_kind().clear_bit()
                .ctr_tx().clear_bit()
                // dtog_rx
                // stat_tx
                .ea().bits(self.index)
        });

        if self.out_buf.is_some() {
            self.set_stat_rx(cs, EndpointStatus::Valid);
        }

        if self.in_buf.is_some() {
            self.set_stat_tx(cs, EndpointStatus::Nak);
        }
    }

    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        let guard = self.in_buf.as_ref().unwrap().try_lock();

        let in_buf = match guard {
            Some(ref b) => b,
            None => { return Err(UsbError::WouldBlock); }
        };

        if buf.len() > in_buf.capacity() {
            return Err(UsbError::BufferOverflow);
        }

        let reg = self.reg();

        match reg.read().stat_tx().bits().into() {
            EndpointStatus::Valid | EndpointStatus::Disabled => return Err(UsbError::WouldBlock),
            _ => {},
        };

        in_buf.write(buf);
        self.descr().count_tx.set(buf.len() as u16 as UsbAccessType);

        interrupt::free(|cs| {
            self.set_stat_tx(cs, EndpointStatus::Valid);
        });

        Ok(buf.len())
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        let guard = self.out_buf.as_ref().unwrap().try_lock();

        let out_buf = match guard {
            Some(ref b) => b,
            None => { return Err(UsbError::WouldBlock); }
        };

        let reg = self.reg();
        let reg_v = reg.read();

        let status: EndpointStatus = reg_v.stat_rx().bits().into();

        if status == EndpointStatus::Disabled || !reg_v.ctr_rx().bit_is_set() {
            return Err(UsbError::WouldBlock);
        }

        let count = (self.descr().count_rx.get() & 0x3ff) as usize;
        if count > buf.len() {
            return Err(UsbError::BufferOverflow);
        }

        out_buf.read(&mut buf[0..count]);

        interrupt::free(|cs| {
            self.clear_ctr_rx(cs);
            self.set_stat_rx(cs, EndpointStatus::Valid);
        });

        Ok(count)
    }

    pub fn read_reg(&self) -> usb::epr::R {
        self.reg().read()
    }

    /*pub fn modify<F>(&self, _cs: CriticalSection, f: F)
        where for<'w> F: FnOnce(&usb::ep0r::R, &'w mut usb::ep0r::W) -> &'w mut usb::ep0r::W
    {
        self.reg().modify(f)
    }*/

    fn clear_toggle_bits(w: &mut usb::epr::W) -> &mut usb::epr::W {
        unsafe {
            w
                .dtog_rx().clear_bit()
                .dtog_tx().clear_bit()
                .stat_rx().bits(0)
                .stat_tx().bits(0)
        }
    }

    pub fn clear_ctr_rx(&self, _cs: &CriticalSection) {
        self.reg().modify(|_, w| Self::clear_toggle_bits(w).ctr_rx().clear_bit());
    }

    pub fn clear_ctr_tx(&self, _cs: &CriticalSection) {
        self.reg().modify(|_, w| Self::clear_toggle_bits(w).ctr_tx().clear_bit());
    }

    pub fn set_stat_rx(&self, _cs: &CriticalSection, status: EndpointStatus) {
        self.reg().modify(|r, w| unsafe {
            Self::clear_toggle_bits(w)
                .stat_rx().bits(r.stat_rx().bits() ^ (status as u8))
        });
    }

    pub fn set_stat_tx(&self, _cs: &CriticalSection, status: EndpointStatus) {
        self.reg().modify(|r, w| unsafe {
            Self::clear_toggle_bits(w)
                .stat_tx().bits(r.stat_tx().bits() ^ (status as u8))
        });
    }
}

/*#[repr(transparent)]
struct EndpointReg(usb::EP0R);

impl EndpointReg {
}*/

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

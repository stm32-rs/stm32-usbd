use core::slice;
use core::mem;
use bare_metal::CriticalSection;
use vcell::VolatileCell;
use cortex_m::interrupt;
use stm32f103xx::{USB, usb};
use usb_device::{Result, UsbError};
use usb_device::endpoint::EndpointType;
use usb_device::utils::AtomicMutex;

type EndpointBuffer = &'static mut [VolatileCell<u32>];

pub const NUM_ENDPOINTS: usize = 8;

#[repr(C)]
struct BufferDescriptor {
    pub addr_tx: VolatileCell<usize>,
    pub count_tx: VolatileCell<usize>,
    pub addr_rx: VolatileCell<usize>,
    pub count_rx: VolatileCell<usize>,
}

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
        size = (size + 1) & !0x01;

        Ok((size, (size << (10 - 1)) as u16))
    } else if size <= 1024 {
        size = (size + 31) & !0x1f;

        Ok((size, (0x8000 | (size << (10 - 5)) as u16)))
    } else {
        Err(UsbError::SizeOverflow)
    }
}

impl Endpoint {
    pub const MEM_START: usize = mem::size_of::<BufferDescriptor>() * NUM_ENDPOINTS;
    pub const MEM_SIZE: usize = 512;
    const MEM_ADDR: *mut VolatileCell<u32> = 0x4000_6000 as *mut VolatileCell<u32>;

    pub fn new(index: u8) -> Endpoint {
        Endpoint {
            out_buf: None,
            in_buf: None,
            ep_type: None,
            index,
        }
    }

    fn make_buf(addr: usize, size: usize)
        -> Option<AtomicMutex<&'static mut [VolatileCell<u32>]>>
    {
        Some(AtomicMutex::new(
            unsafe {
                slice::from_raw_parts_mut(
                    Self::MEM_ADDR.offset((addr >> 1) as isize),
                    size >> 1)
            }
        ))
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

    pub fn set_out_buf(&mut self, addr: usize, size_and_bits: (usize, u16)) {
        self.out_buf = Self::make_buf(addr, size_and_bits.0);

        let descr = self.descr();
        descr.addr_rx.set(addr);
        descr.count_rx.set(size_and_bits.1 as usize);
    }

    pub fn is_in_buf_set(&self) -> bool {
        self.in_buf.is_some()
    }

    pub fn set_in_buf(&mut self, addr: usize, max_packet_size: usize) {
        self.in_buf = Self::make_buf(addr, max_packet_size);

        let descr = self.descr();
        descr.addr_tx.set(addr);
        descr.count_tx.set(0);
    }

    /*fn mem(&self) -> &'static [VolatileCell<u32>] {
        unsafe { slice::from_raw_parts(self.addr as *const VolatileCell<u32>, self.size) }
    }*/

    fn descr(&self) -> &'static BufferDescriptor {
        unsafe { &*(Self::MEM_ADDR as *const BufferDescriptor).offset(self.index as isize) }
    }

    fn reg(&self) -> &'static usb::EP0R {
        unsafe { &*(&(*USB::ptr()).ep0r as *const usb::EP0R).offset(self.index as isize) }
    }

    pub fn configure(&self, cs: &CriticalSection) {
        let ep_type = match self.ep_type {
            Some(t) => t,
            None => { return },
        };

        self.reg().modify(|_, w|
            Self::clear_toggle_bits(w)
                .ctr_rx().clear_bit()
                // dtog_rx
                // stat_rx
                .ep_type().bits(ep_type.bits())
                .ep_kind().clear_bit()
                .ctr_tx().clear_bit()
                // dtog_rx
                // stat_tx
                .ea().bits(self.index));

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
            None => { return Err(UsbError::Busy); }
        };

        if buf.len() > in_buf.len() << 1 {
            return Err(UsbError::BufferOverflow);
        }

        let reg = self.reg();

        match reg.read().stat_tx().bits().into() {
            EndpointStatus::Valid => return Err(UsbError::Busy),
            EndpointStatus::Disabled => return Err(UsbError::InvalidEndpoint),
            _ => {},
        };

        self.write_mem(in_buf, buf);
        self.descr().count_tx.set(buf.len());

        interrupt::free(|cs| {
            self.set_stat_tx(cs, EndpointStatus::Valid);
        });

        Ok(buf.len())
    }

    fn write_mem(&self, mem: &[VolatileCell<u32>], mut buf: &[u8]) {
        let mut addr = 0;

        while buf.len() >= 2 {
            mem[addr].set((buf[0] as u16 | ((buf[1] as u16) << 8)) as u32);
            addr += 1;

            buf = &buf[2..];
        }

        if buf.len() > 0 {
            mem[addr].set(buf[0] as u32);
        }
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        let guard = self.out_buf.as_ref().unwrap().try_lock();

        let out_buf = match guard {
            Some(ref b) => b,
            None => { return Err(UsbError::Busy); }
        };

        let reg = self.reg();
        let reg_v = reg.read();

        let status: EndpointStatus = reg_v.stat_rx().bits().into();

        if status == EndpointStatus::Disabled {
            return Err(UsbError::InvalidEndpoint);
        }

        if !reg_v.ctr_rx().bit_is_set() {
            return Err(UsbError::NoData);
        }

        let count = self.descr().count_rx.get() & 0x3f;
        if count > buf.len() {
            return Err(UsbError::BufferOverflow);
        }

        self.read_mem(out_buf, &mut buf[0..count]);

        interrupt::free(|cs| {
            self.clear_ctr_rx(cs);
            self.set_stat_rx(cs, EndpointStatus::Valid);
        });

        Ok(count)
    }

    fn read_mem(&self, mem: &[VolatileCell<u32>], mut buf: &mut [u8]) {
        let mut addr = 0;

        while buf.len() >= 2 {
            let word = mem[addr].get();

            buf[0] = word as u8;
            buf[1] = (word >> 8) as u8;

            addr += 1;

            buf = &mut {buf}[2..];
        }

        if buf.len() > 0 {
            buf[0] = mem[addr].get() as u8;
        }
    }

    pub fn read_reg(&self) -> usb::ep0r::R {
        self.reg().read()
    }

    /*pub fn modify<F>(&self, _cs: CriticalSection, f: F)
        where for<'w> F: FnOnce(&usb::ep0r::R, &'w mut usb::ep0r::W) -> &'w mut usb::ep0r::W
    {
        self.reg().modify(f)
    }*/

    fn clear_toggle_bits(w: &mut usb::ep0r::W) -> &mut usb::ep0r::W {
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
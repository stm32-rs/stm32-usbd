use usb_device::{Result, UsbError, EndpointType};
use stm32f103xx::usb;
use core::mem;
use vcell::VolatileCell;

pub const NUM_ENDPOINTS: usize = 8;

#[repr(C)]
pub struct BufDescriptor {
    pub addr_tx: VolatileCell<usize>,
    pub count_tx: VolatileCell<usize>,
    pub addr_rx: VolatileCell<usize>,
    pub count_rx: VolatileCell<usize>,
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

pub struct PacketMemory {
    next: usize,
}

impl PacketMemory {
    const ADDR: usize = 0x4000_6000;
    const SIZE: usize = 512;
    const START: usize = mem::size_of::<BufDescriptor>() * NUM_ENDPOINTS;

    pub fn new() -> PacketMemory {
        PacketMemory {
            next: PacketMemory::START
        }
    }

    pub fn reset(&mut self) {
        self.next = PacketMemory::START;
    }

    fn mem() -> &'static [VolatileCell<u32>; PacketMemory::SIZE/2] {
        return unsafe { &*(PacketMemory::ADDR as *const [VolatileCell<u32>; PacketMemory::SIZE/2]) };
    }

    pub fn alloc(&mut self, size: usize) -> Result<usize> {
        assert!(size & 1 == 0);

        let addr = self.next;
        if addr + size > PacketMemory::SIZE {
            return Err(UsbError::SizeOverflow);
        }

        self.next += size;

        Ok(addr)
    }

    pub fn write(&self, mut addr: usize, mut buf: &[u8]) {
        let mem = PacketMemory::mem();

        addr >>= 1;

        while buf.len() >= 2 {
            mem[addr].set((buf[0] as u16 | ((buf[1] as u16) << 8)) as u32);
            addr += 1;

            buf = &buf[2..];
        }

        if buf.len() > 0 {
            mem[addr].set(buf[0] as u32);
        }
    }

    pub fn read(&self, mut addr: usize, mut buf: &mut [u8]) {
        let mem = PacketMemory::mem();

        addr >>= 1;

        while buf.len() >= 2 {
            let word = mem[addr].get();

            buf[0] = word as u8;
            buf[1] = (word >> 8) as u8;

            addr += 1;

            buf = &mut {buf}[2..];
        }

        if buf.len() > 0 {
            buf[0] = mem[0].get() as u8;
        }
    }

    pub fn descrs(&self) -> &mut [BufDescriptor; NUM_ENDPOINTS] {
        return unsafe { &mut *(PacketMemory::ADDR as *mut [BufDescriptor; NUM_ENDPOINTS]) };
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

#[repr(transparent)]
pub struct EpReg(usb::EP0R);

/// Wraps a USB endpoint register to implement the "toggle to set" logic
impl EpReg {
    pub fn read(&self) -> usb::ep0r::R {
        self.0.read()
    }

    pub fn configure(&self, ep_type: EndpointType, addr: u8) {
        self.0.modify(|_, w|
            Self::clear_toggle_bits(w)
                .ctr_rx().clear_bit()
                // dtog_rx
                // stat_rx
                .ep_type().bits(ep_type.bits())
                .ep_kind().clear_bit()
                .ctr_tx().clear_bit()
                // dtog_rx
                // stat_tx
                .ea().bits(addr));
    }

    fn clear_toggle_bits(w: &mut usb::ep0r::W) -> &mut usb::ep0r::W {
        unsafe {
            w
                .dtog_rx().clear_bit()
                .dtog_tx().clear_bit()
                .stat_rx().bits(0)
                .stat_tx().bits(0)
        }
    }

    pub fn clear_ctr_rx(&self) {
        self.0.modify(|_, w| Self::clear_toggle_bits(w).ctr_rx().clear_bit());
    }

    pub fn clear_ctr_tx(&self) {
        self.0.modify(|_, w| Self::clear_toggle_bits(w).ctr_tx().clear_bit());
    }

    pub fn set_stat_rx(&self, status: EndpointStatus) {
        self.0.modify(|r, w| unsafe {
            Self::clear_toggle_bits(w)
                .stat_rx().bits(r.stat_rx().bits() ^ (status as u8))
        });
    }

    pub fn set_stat_tx(&self, status: EndpointStatus) {
        self.0.modify(|r, w| unsafe {
            Self::clear_toggle_bits(w)
                .stat_tx().bits(r.stat_tx().bits() ^ (status as u8))
        });
    }
}
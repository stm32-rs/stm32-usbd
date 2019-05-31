use core::{slice, mem};
use vcell::VolatileCell;
use crate::target::{UsbAccessType, EP_MEM_ADDR, EP_MEM_SIZE, NUM_ENDPOINTS};
use usb_device::{Result, UsbError};

pub struct EndpointBuffer(&'static mut [VolatileCell<UsbAccessType>]);

impl EndpointBuffer {
    const MEM_ADDR: *mut VolatileCell<UsbAccessType> = EP_MEM_ADDR as *mut VolatileCell<UsbAccessType>;

    pub fn new(offset_bytes: usize, size_bytes: usize) -> Self {
        let mem = unsafe {
            slice::from_raw_parts_mut(
                Self::MEM_ADDR.offset((offset_bytes >> 1) as isize),
                size_bytes >> 1)
        };
        Self(mem)
    }

    #[inline(always)]
    fn read_word(&self, index: usize) -> u16 {
        (self.0[index].get() & 0xffff) as u16
    }

    #[inline(always)]
    fn write_word(&self, index: usize, value: u16) {
        self.0[index].set(value as UsbAccessType);
    }

    pub fn read(&self, mut buf: &mut [u8]) {
        let mut index = 0;

        while buf.len() >= 2 {
            let word = self.read_word(index);

            buf[0] = (word & 0xff) as u8;
            buf[1] = (word >> 8) as u8;

            index += 1;

            buf = &mut buf[2..];
        }

        if buf.len() > 0 {
            buf[0] = (self.read_word(index) & 0xff) as u8;
        }
    }

    pub fn write(&self, mut buf: &[u8]) {
        let mut index = 0;

        while buf.len() >= 2 {
            let value: u16 = buf[0] as u16 | ((buf[1] as u16) << 8);
            self.write_word(index, value);
            index += 1;

            buf = &buf[2..];
        }

        if buf.len() > 0 {
            self.write_word(index, buf[0] as u16);
        }
    }

    pub fn offset(&self) -> usize {
        let buffer_address = self.0.as_ptr() as usize;
        let index = (buffer_address - EP_MEM_ADDR) / mem::size_of::<UsbAccessType>();
        index << 1
    }

    pub fn capacity(&self) -> usize {
        self.0.len() << 1
    }
}

#[repr(C)]
pub struct BufferDescriptor {
    pub addr_tx: VolatileCell<UsbAccessType>,
    pub count_tx: VolatileCell<UsbAccessType>,
    pub addr_rx: VolatileCell<UsbAccessType>,
    pub count_rx: VolatileCell<UsbAccessType>,
}

pub struct EndpointMemoryAllocator {
    next_free_offset: usize,
}

impl EndpointMemoryAllocator {
    pub fn new() -> Self {
        Self {
            next_free_offset: NUM_ENDPOINTS * 8
        }
    }

    pub fn allocate_buffer(&mut self, size: usize) -> Result<EndpointBuffer> {
        assert!(size & 1 == 0);
        assert!(size < EP_MEM_SIZE);

        let offset = self.next_free_offset;
        if offset as usize + size > EP_MEM_SIZE {
            return Err(UsbError::EndpointMemoryOverflow);
        }

        self.next_free_offset += size;

        Ok(EndpointBuffer::new(offset, size))
    }

    pub fn buffer_descriptor(index: u8) -> &'static BufferDescriptor {
        unsafe { &*(EP_MEM_ADDR as *const BufferDescriptor).offset(index as isize) }
    }
}

use crate::endpoint::NUM_ENDPOINTS;
use crate::UsbPeripheral;
use core::marker::PhantomData;
use core::slice;
use usb_device::{Result, UsbError};
use vcell::VolatileCell;


pub struct EndpointBuffer<USB> {
    mem: &'static mut [VolatileCell<u16>],
    marker: PhantomData<USB>
}

impl<USB: UsbPeripheral> EndpointBuffer<USB> {
    pub fn new(offset_bytes: usize, size_bytes: usize) -> Self {
        let ep_mem_ptr = USB::EP_MEMORY as *mut VolatileCell<u16>;

        let offset_words = offset_bytes >> 1;
        let count_words = size_bytes >> 1;
        let offset_u16_words;
        let count_u16_words;
        if USB::EP_MEMORY_ACCESS_2X16 {
            offset_u16_words = offset_words;
            count_u16_words = count_words;
        } else {
            offset_u16_words = offset_words * 2;
            count_u16_words = count_words * 2;
        };

        unsafe {
            let mem = slice::from_raw_parts_mut(ep_mem_ptr.add(offset_u16_words), count_u16_words);
            Self {
                mem,
                marker: PhantomData,
            }
        }
    }

    #[inline(always)]
    fn read_word(&self, index: usize) -> u16 {
        if USB::EP_MEMORY_ACCESS_2X16 {
            self.mem[index].get()
        } else {
            self.mem[index * 2].get()
        }
    }

    #[inline(always)]
    fn write_word(&self, index: usize, value: u16) {
        if USB::EP_MEMORY_ACCESS_2X16 {
            self.mem[index].set(value);
        } else {
            self.mem[index * 2].set(value);
        }
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

    pub fn offset(&self) -> u16 {
        let buffer_address = self.mem.as_ptr() as usize;
        let word_size = if USB::EP_MEMORY_ACCESS_2X16 {
            2
        } else {
            4
        };
        let index = (buffer_address - USB::EP_MEMORY as usize) / word_size;
        (index << 1) as u16
    }

    pub fn capacity(&self) -> usize {
        let len_words = if USB::EP_MEMORY_ACCESS_2X16 {
            self.mem.len()
        } else {
            self.mem.len() / 2
        };
        len_words << 1
    }
}

#[repr(C)]
pub struct BufferDescriptor<USB> {
    ptr: *const VolatileCell<u16>,
    marker: PhantomData<USB>,
}

impl<USB: UsbPeripheral> BufferDescriptor<USB> {
    #[inline(always)]
    fn field(&self, index: usize) -> &'static VolatileCell<u16> {
        let mul = if USB::EP_MEMORY_ACCESS_2X16 {
            1
        } else {
            2
        };
        unsafe { &*(self.ptr.add(index * mul)) }
    }

    #[inline(always)]
    pub fn addr_tx(&self) -> &'static VolatileCell<u16> {
        self.field(0)
    }

    #[inline(always)]
    pub fn count_tx(&self) -> &'static VolatileCell<u16> {
        self.field(1)
    }

    #[inline(always)]
    pub fn addr_rx(&self) -> &'static VolatileCell<u16> {
        self.field(2)
    }

    #[inline(always)]
    pub fn count_rx(&self) -> &'static VolatileCell<u16> {
        self.field(3)
    }
}

pub struct EndpointMemoryAllocator<USB> {
    next_free_offset: usize,
    _marker: PhantomData<USB>,
}

impl<USB: UsbPeripheral> EndpointMemoryAllocator<USB> {
    pub fn new() -> Self {
        Self {
            next_free_offset: NUM_ENDPOINTS * 8,
            _marker: PhantomData,
        }
    }

    pub fn allocate_buffer(&mut self, size: usize) -> Result<EndpointBuffer<USB>> {
        assert_eq!(size & 1, 0);
        assert!(size < USB::EP_MEMORY_SIZE);

        let offset = self.next_free_offset;
        if offset as usize + size > USB::EP_MEMORY_SIZE {
            return Err(UsbError::EndpointMemoryOverflow);
        }

        self.next_free_offset += size;

        Ok(EndpointBuffer::new(offset, size))
    }

    pub fn buffer_descriptor(index: u8) -> BufferDescriptor<USB> {
        let mul = if USB::EP_MEMORY_ACCESS_2X16 {
            1
        } else {
            2
        };

        unsafe {
            let ptr = (USB::EP_MEMORY as *const VolatileCell<u16>).add((index as usize) * 4 * mul);
            BufferDescriptor {
                ptr,
                marker: Default::default()
            }
        }
    }
}

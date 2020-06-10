use crate::endpoint::NUM_ENDPOINTS;
use crate::UsbPeripheral;
use core::marker::PhantomData;
use core::{mem, slice};
use usb_device::{Result, UsbError};
use vcell::VolatileCell;

#[cfg(feature = "ram_access_1x16")]
pub type UsbAccessType = u32;
#[cfg(feature = "ram_access_2x16")]
pub type UsbAccessType = u16;

pub struct EndpointBuffer<USB> {
    mem: &'static mut [VolatileCell<UsbAccessType>],
    marker: PhantomData<USB>
}

impl<USB: UsbPeripheral> EndpointBuffer<USB> {
    pub fn new(offset_bytes: usize, size_bytes: usize) -> Self {
        let ep_mem_ptr = USB::EP_MEMORY as *mut VolatileCell<UsbAccessType>;

        let mem =
            unsafe { slice::from_raw_parts_mut(ep_mem_ptr.offset((offset_bytes >> 1) as isize), size_bytes >> 1) };
        Self {
            mem,
            marker: PhantomData,
        }
    }

    #[inline(always)]
    fn read_word(&self, index: usize) -> u16 {
        (self.mem[index].get() & 0xffff) as u16
    }

    #[inline(always)]
    fn write_word(&self, index: usize, value: u16) {
        self.mem[index].set(value as UsbAccessType);
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
        self.mem.len() << 1
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

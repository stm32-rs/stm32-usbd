use crate::endpoint::NUM_ENDPOINTS;
use crate::{UsbPeripheral, Word};
use core::marker::PhantomData;
use core::slice;
use usb_device::{Result, UsbError};
use vcell::VolatileCell;

pub struct EndpointBuffer<USB: UsbPeripheral> {
    mem: &'static mut [VolatileCell<USB::Word>],
    marker: PhantomData<USB>,
}

impl<USB: UsbPeripheral> EndpointBuffer<USB> {
    pub fn new(offset_bytes: usize, size_bytes: usize) -> Self {
        let ep_mem_ptr = USB::EP_MEMORY as *mut VolatileCell<_>;

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
    fn read_word(&self, index: usize) -> USB::Word {
        if USB::EP_MEMORY_ACCESS_2X16 {
            self.mem[index].get()
        } else {
            self.mem[index * 2].get()
        }
    }

    #[inline(always)]
    fn write_word(&self, index: usize, value: USB::Word) {
        if USB::EP_MEMORY_ACCESS_2X16 {
            self.mem[index].set(value);
        } else {
            self.mem[index * 2].set(value);
        }
    }

    pub fn read(&self, buf: &mut [u8]) {
        let mut index = 0;
        let mut writer = buf.into_iter().peekable();

        while writer.peek().is_some() {
            self.read_word(index).write_to_iter_le(&mut writer);
            index += 1;
        }
    }

    pub fn write(&self, buf: &[u8]) {
        let mut index = 0;
        let mut reader = buf.into_iter().peekable();

        while reader.peek().is_some() {
            let value = Word::from_iter_le(&mut reader);
            self.write_word(index, value);
            index += 1;
        }
    }

    pub fn offset(&self) -> u16 {
        let buffer_address = self.mem.as_ptr() as usize;
        let word_size = if USB::EP_MEMORY_ACCESS_2X16 { 2 } else { 4 };
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
pub struct BufferDescriptor<USB: UsbPeripheral> {
    ptr: *const VolatileCell<USB::Word>,
    marker: PhantomData<USB>,
}

impl<USB: UsbPeripheral> BufferDescriptor<USB> {
    #[inline(always)]
    fn field(&self, index: usize) -> &'static VolatileCell<USB::Word> {
        let mul = if USB::EP_MEMORY_ACCESS_2X16 { 1 } else { 2 };
        unsafe { &*(self.ptr.add(index * mul)) }
    }

    #[inline(always)]
    pub fn set_tx(&self, address: u16, count: u16) {
        self.field(0) // addr
        self.field(1) // count (msb in 32bit)
    }

    #[inline(always)]
    pub fn get_rx(&self) -> (u16, u16) {
        self.field(2) // addr
        self.field(3) // count(msb in 32bit)
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
        let mul = if USB::EP_MEMORY_ACCESS_2X16 { 1 } else { 2 };

        unsafe {
            let ptr = (USB::EP_MEMORY as *const VolatileCell<USB::Word>).add((index as usize) * 4 * mul);
            BufferDescriptor {
                ptr,
                marker: Default::default(),
            }
        }
    }
}

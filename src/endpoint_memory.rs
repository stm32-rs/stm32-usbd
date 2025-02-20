use crate::endpoint::NUM_ENDPOINTS;
use crate::{SramAccessScheme, UsbPeripheral, Word};
use core::marker::PhantomData;
use core::mem;
use usb_device::{Result, UsbError};

pub struct EndpointBuffer<USB: UsbPeripheral> {
    ptr: *mut <USB::SramAccessScheme as SramAccessScheme>::Word,
    word_count: usize,
    marker: PhantomData<USB>,
}

unsafe impl<USB: UsbPeripheral> Send for EndpointBuffer<USB> {}

impl<USB: UsbPeripheral> EndpointBuffer<USB> {
    pub fn new(offset_bytes: usize, size_bytes: usize) -> Self {
        let ep_mem_ptr = USB::EP_MEMORY as *mut <USB::SramAccessScheme as SramAccessScheme>::Word;

        let word_size = Self::word_size();
        let offset_words = offset_bytes / word_size;
        let word_count = size_bytes / word_size;

        let offset_words = offset_words * USB::SramAccessScheme::ADDRESS_MULTIPLIER;
        let word_count = word_count * USB::SramAccessScheme::ADDRESS_MULTIPLIER;

        unsafe {
            let ptr = ep_mem_ptr.add(offset_words);
            Self {
                ptr,
                word_count,
                marker: PhantomData,
            }
        }
    }

    fn word_size() -> usize {
        mem::size_of::<<USB::SramAccessScheme as SramAccessScheme>::Word>()
    }

    #[inline(always)]
    fn read_word(&self, index: usize) -> <USB::SramAccessScheme as SramAccessScheme>::Word {
        let index = index * USB::SramAccessScheme::ADDRESS_MULTIPLIER;
        assert!(index < self.word_count);
        unsafe { self.ptr.add(index).read_volatile() }
    }

    #[inline(always)]
    fn write_word(&self, index: usize, value: <USB::SramAccessScheme as SramAccessScheme>::Word) {
        let index = index * USB::SramAccessScheme::ADDRESS_MULTIPLIER;
        assert!(index < self.word_count);
        unsafe {
            self.ptr.add(index).write_volatile(value);
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
        let buffer_address = self.ptr as usize;
        let word_size = Self::word_size();
        let offset_per_word = word_size * USB::SramAccessScheme::ADDRESS_MULTIPLIER;
        let index = (buffer_address - USB::EP_MEMORY as usize) / offset_per_word;
        (index * word_size) as u16
    }

    /// Capacity in bytes
    pub fn capacity(&self) -> usize {
        let len_words = self.word_count / USB::SramAccessScheme::ADDRESS_MULTIPLIER;
        let word_size = Self::word_size();
        len_words * word_size
    }
}

#[repr(C)]
pub struct BufferDescriptor<USB: UsbPeripheral> {
    ptr: *mut <USB::SramAccessScheme as SramAccessScheme>::Word,
    marker: PhantomData<USB>,
}

pub struct DescriptorPart {
    pub address: u16,
    pub count: u16,
}

impl<USB: UsbPeripheral> BufferDescriptor<USB> {
    #[inline(always)]
    pub fn set_tx(&self, address: u16, count: u16) {
        unsafe {
            USB::SramAccessScheme::set(self.ptr, crate::AccessType::Tx, address, count);
        }
    }

    #[inline(always)]
    pub fn get_tx(&self) -> DescriptorPart {
        let (address, count) = unsafe { USB::SramAccessScheme::read(self.ptr, crate::AccessType::Tx) };
        DescriptorPart { address, count }
    }

    #[inline(always)]
    pub fn set_rx(&self, address: u16, count: u16) {
        unsafe {
            USB::SramAccessScheme::set(self.ptr, crate::AccessType::Rx, address, count);
        }
    }

    #[inline(always)]
    pub fn get_rx(&self) -> DescriptorPart {
        let (address, count) = unsafe { USB::SramAccessScheme::read(self.ptr, crate::AccessType::Rx) };
        DescriptorPart { address, count }
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
        let mul = USB::SramAccessScheme::ADDRESS_MULTIPLIER;
        let word_size = mem::size_of::<<USB::SramAccessScheme as SramAccessScheme>::Word>();

        // 4xu16=8Bytes worth of data per descriptor
        let descriptor_size_bytes = 8;
        let offset_per_descriptor = descriptor_size_bytes * mul / word_size;
        unsafe {
            let ptr = (USB::EP_MEMORY as *mut <USB::SramAccessScheme as SramAccessScheme>::Word)
                .add((index as usize) * offset_per_descriptor);
            BufferDescriptor {
                ptr,
                marker: Default::default(),
            }
        }
    }
}

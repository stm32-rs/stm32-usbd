use crate::endpoint::NUM_ENDPOINTS;
use crate::UsbPeripheral;
use core::slice;
use core::{marker::PhantomData, mem};
use usb_device::{Result, UsbError};
use vcell::VolatileCell;

/// Different endpoint memory access schemes
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum MemoryAccess {
    /// 16x1 bits per word. Each 32-bit word is accessed as one 16-bit half-word. The second half-word of the word is ignored.
    ///
    /// This matches the behavior of `EP_MEMORY_ACCESS_2X16 = false` from previous versions of this library.
    Word16x1,
    /// 16x2 bits per word. Each 32-bit word is accessed as two 16-bit half-words.
    ///
    /// This matches the behavior of `EP_MEMORY_ACCESS_2X16 = true` from previous versions of this library.
    Word16x2,
    /// 32x1 bits per word. Each 32-bit word is accessed as one 32-bit word.
    Word32x1,
}

impl MemoryAccess {
    /// Value to multiply offsets within the EP memory by when calculating address to read or write to.
    #[inline(always)]
    const fn offset_multiplier(self) -> usize {
        match self {
            MemoryAccess::Word16x1 => 2,
            MemoryAccess::Word16x2 | MemoryAccess::Word32x1 => 1,
        }
    }

    /// Size of unit used when reading and writing EP memory, in bytes.
    #[inline(always)]
    const fn unit_size(self) -> usize {
        match self {
            MemoryAccess::Word16x1 | MemoryAccess::Word16x2 => 2,
            MemoryAccess::Word32x1 => 4,
        }
    }
}

pub struct EndpointBuffer<USB> {
    mem_ptr: *mut (),
    mem_len: usize,
    marker: PhantomData<USB>,
}

unsafe impl<USB> Send for EndpointBuffer<USB> {}

impl<USB: UsbPeripheral> EndpointBuffer<USB> {
    pub fn new(offset_bytes: usize, size_bytes: usize) -> Self {
        let mem_offset_bytes = offset_bytes * USB::EP_MEMORY_ACCESS.offset_multiplier();
        let mem_len = size_bytes * USB::EP_MEMORY_ACCESS.offset_multiplier() / USB::EP_MEMORY_ACCESS.unit_size();

        let mem_ptr = unsafe { USB::EP_MEMORY.byte_add(mem_offset_bytes).cast_mut() };
        Self {
            mem_ptr,
            mem_len,
            marker: PhantomData,
        }
    }

    #[inline(always)]
    fn get_mem_slice<T>(&self) -> &mut [VolatileCell<T>] {
        unsafe { slice::from_raw_parts_mut(self.mem_ptr.cast(), self.mem_len) }
    }

    pub fn read(&self, mut buf: &mut [u8]) {
        if USB::EP_MEMORY_ACCESS == MemoryAccess::Word32x1 {
            let mem = self.get_mem_slice::<u32>();

            let mut index = 0;

            while buf.len() >= 4 {
                let value = mem[index].get().to_ne_bytes();
                index += USB::EP_MEMORY_ACCESS.offset_multiplier();

                buf[0..4].copy_from_slice(&value);
                buf = &mut buf[4..];
            }

            if buf.len() > 0 {
                let value = mem[index].get().to_ne_bytes();
                buf.copy_from_slice(&value[0..buf.len()]);
            }
        } else {
            let mem = self.get_mem_slice::<u16>();

            let mut index = 0;

            while buf.len() >= 2 {
                let value = mem[index].get().to_ne_bytes();
                index += USB::EP_MEMORY_ACCESS.offset_multiplier();

                buf[0..2].copy_from_slice(&value);
                buf = &mut buf[2..];
            }

            if buf.len() > 0 {
                let value = mem[index].get().to_ne_bytes();
                buf.copy_from_slice(&value[0..buf.len()]);
            }
        }
    }

    pub fn write(&self, mut buf: &[u8]) {
        if USB::EP_MEMORY_ACCESS == MemoryAccess::Word32x1 {
            let mem = self.get_mem_slice::<u32>();

            let mut index = 0;

            while buf.len() >= 4 {
                let mut value = [0; 4];
                value.copy_from_slice(&buf[0..4]);
                buf = &buf[4..];
                
                mem[index].set(u32::from_ne_bytes(value));
                index += USB::EP_MEMORY_ACCESS.offset_multiplier();
            }

            if buf.len() > 0 {
                let mut value = [0; 4];
                value[0..buf.len()].copy_from_slice(buf);
                mem[index].set(u32::from_ne_bytes(value));
            }
        } else {
            let mem = self.get_mem_slice::<u16>();

            let mut index = 0;

            while buf.len() >= 2 {
                let mut value = [0; 2];
                value.copy_from_slice(&buf[0..2]);
                buf = &buf[2..];

                mem[index].set(u16::from_ne_bytes(value));
                index += USB::EP_MEMORY_ACCESS.offset_multiplier();
            }

            if buf.len() > 0 {
                let mut value = [0; 2];
                value[0..buf.len()].copy_from_slice(buf);
                mem[index].set(u16::from_ne_bytes(value));
            }
        }
    }

    pub fn offset(&self) -> u16 {
        let offset_bytes = self.mem_ptr as usize - USB::EP_MEMORY as usize;
        (offset_bytes / USB::EP_MEMORY_ACCESS.offset_multiplier()) as u16
    }

    pub fn capacity(&self) -> usize {
        self.mem_len * USB::EP_MEMORY_ACCESS.unit_size() / USB::EP_MEMORY_ACCESS.offset_multiplier()
    }
}

pub struct Field<USB> {
    ptr: *const (),
    marker: PhantomData<USB>,
}

impl<USB: UsbPeripheral> Field<USB> {
    #[inline(always)]
    pub fn get(&self) -> u16 {
        if USB::EP_MEMORY_ACCESS == MemoryAccess::Word32x1 {
            let unaligned_offset = self.ptr as usize & 0b11;
            let cell: &VolatileCell<u32> = unsafe { &*self.ptr.byte_sub(unaligned_offset).cast() };
            let value: [u16; 2] = unsafe { mem::transmute(cell.get()) };
            value[unaligned_offset >> 1]
        } else {
            let cell: &VolatileCell<u16> = unsafe { &*self.ptr.cast() };
            cell.get()
        }
    }

    #[inline(always)]
    pub fn set(&self, value: u16) {
        if USB::EP_MEMORY_ACCESS == MemoryAccess::Word32x1 {
            let unaligned_offset = self.ptr as usize & 0b11;
            let cell: &VolatileCell<u32> = unsafe { &*self.ptr.byte_sub(unaligned_offset).cast() };
            let mut previous_value: [u16; 2] = unsafe { mem::transmute(cell.get()) };
            previous_value[unaligned_offset >> 1] = value;
            cell.set(unsafe { mem::transmute(previous_value) });
        } else {
            let cell: &VolatileCell<u16> = unsafe { &*self.ptr.cast() };
            cell.set(value);
        }
    }
}

#[repr(C)]
pub struct BufferDescriptor<USB> {
    ptr: *const (),
    marker: PhantomData<USB>,
}

impl<USB: UsbPeripheral> BufferDescriptor<USB> {
    #[inline(always)]
    fn field(&self, index: usize) -> Field<USB> {
        let mul = USB::EP_MEMORY_ACCESS.offset_multiplier();
        let ptr = unsafe { self.ptr.byte_add(index * 2 * mul) };
        Field {
            ptr,
            marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn addr_tx(&self) -> Field<USB> {
        self.field(0)
    }

    #[inline(always)]
    pub fn count_tx(&self) -> Field<USB> {
        self.field(1)
    }

    #[inline(always)]
    pub fn addr_rx(&self) -> Field<USB> {
        self.field(2)
    }

    #[inline(always)]
    pub fn count_rx(&self) -> Field<USB> {
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
        let mul = USB::EP_MEMORY_ACCESS.offset_multiplier();
        let ptr = unsafe { USB::EP_MEMORY.byte_add((index as usize) * 8 * mul).cast() };
        BufferDescriptor {
            ptr,
            marker: Default::default(),
        }
    }
}

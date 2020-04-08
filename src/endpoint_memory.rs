use crate::endpoint::NUM_ENDPOINTS;
use crate::UsbPeripheral;
use core::marker::PhantomData;
use core::slice;
use usb_device::{Result, UsbError};
use vcell::VolatileCell;

#[cfg(feature = "ram_access_1x16")]
pub type UsbAccessType = u32;
#[cfg(feature = "ram_access_2x16")]
pub type UsbAccessType = u16;

pub struct EndpointBuffer(&'static mut [VolatileCell<UsbAccessType>]);

impl EndpointBuffer {
    pub fn new<USB: UsbPeripheral>(offset_bytes: usize, size_bytes: usize) -> Self {
        let ep_mem_ptr = USB::EP_MEMORY as *mut VolatileCell<UsbAccessType>;

        let mem =
            unsafe { slice::from_raw_parts_mut(ep_mem_ptr.offset((offset_bytes >> 1) as isize), size_bytes >> 1) };
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

    /*pub fn offset<USB: UsbPeripheral>(&self) -> usize {
        let buffer_address = self.0.as_ptr() as usize;
        let index = (buffer_address - USB::EP_MEMORY as usize) / mem::size_of::<UsbAccessType>();
        index << 1
    }*/

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

/*impl BufferDescriptor {
    fn get<USB: UsbPeripheral>(index: u8) -> &'static BufferDescriptor{
        unsafe { &*(USB::EP_MEMORY as *const BufferDescriptor).offset(index as isize) }
    }
}*/

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

    pub fn allocate_buffer(&mut self, size: usize) -> Result<usize> {
        assert_eq!(size & 1, 0);
        assert!(size < USB::EP_MEMORY_SIZE);

        let offset = self.next_free_offset;
        if offset as usize + size > USB::EP_MEMORY_SIZE {
            return Err(UsbError::EndpointMemoryOverflow);
        }

        self.next_free_offset += size;

        Ok(offset)
    }

    pub fn buffer_descriptor(index: u8) -> &'static BufferDescriptor {
        unsafe { &*(USB::EP_MEMORY as *const BufferDescriptor).offset(index as isize) }
    }
}

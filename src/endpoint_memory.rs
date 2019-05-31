use core::slice;
use vcell::VolatileCell;
use crate::target::UsbAccessType;

pub struct EndpointBuffer(&'static mut [VolatileCell<UsbAccessType>]);

impl EndpointBuffer {
    const MEM_ADDR: *mut VolatileCell<UsbAccessType> = 0x4000_6000 as *mut VolatileCell<UsbAccessType>;

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

    pub fn capacity(&self) -> usize {
        self.0.len() << 1
    }
}

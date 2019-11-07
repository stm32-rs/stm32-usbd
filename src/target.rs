//! Target-specific definitions

#[cfg(feature = "ram_access_1x16")]
pub type UsbAccessType = u32;
#[cfg(feature = "ram_access_2x16")]
pub type UsbAccessType = u16;

#[cfg(not(feature = "ram_addr_40006c00"))]
pub const EP_MEM_ADDR: usize = 0x4000_6000;
#[cfg(feature = "ram_addr_40006c00")]
pub const EP_MEM_ADDR: usize = 0x4000_6C00;

#[cfg(feature = "ram_size_512")]
pub const EP_MEM_SIZE: usize = 512;
#[cfg(feature = "ram_size_1024")]
pub const EP_MEM_SIZE: usize = 1024;

pub const NUM_ENDPOINTS: usize = 8;

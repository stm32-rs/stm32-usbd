//! Target-specific definitions

#[cfg(feature = "ram_access_1x16")]
pub type UsbAccessType = u32;
#[cfg(feature = "ram_access_2x16")]
pub type UsbAccessType = u16;

pub const NUM_ENDPOINTS: usize = 8;

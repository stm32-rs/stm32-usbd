//! Target-specific definitions

// Export HAL
#[cfg(feature = "stm32f0")]
pub use stm32f0xx_hal as hal;
#[cfg(feature = "stm32f1")]
pub use stm32f1xx_hal as hal;
#[cfg(feature = "stm32f3")]
pub use stm32f3xx_hal as hal;
#[cfg(feature = "stm32l0")]
pub use stm32l0xx_hal as hal;
#[cfg(feature = "stm32l4")]
pub use stm32l4xx_hal as hal;

// Use bundled register definitions instead of device-specific ones
// This should work because register definitions from newer chips seem to be
// compatible with definitions for older ones.
pub use crate::pac::usb;

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

#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FNR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct FNR {
    bits: u16,
}
impl FNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LSOFR {
    bits: u8,
}
impl LSOFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `LCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCKR {
    #[doc = "the frame timer remains in this state until an USB reset or USB suspend event occurs"]
    LOCKED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl LCKR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LCKR::LOCKED => true,
            LCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LCKR {
        match value {
            true => LCKR::LOCKED,
            i => LCKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline]
    pub fn is_locked(&self) -> bool {
        *self == LCKR::LOCKED
    }
}
#[doc = "Possible values of the field `RXDM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMR {
    #[doc = "received data minus upstream port data line"]
    RECEIVED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RXDMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RXDMR::RECEIVED => true,
            RXDMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDMR {
        match value {
            true => RXDMR::RECEIVED,
            i => RXDMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline]
    pub fn is_received(&self) -> bool {
        *self == RXDMR::RECEIVED
    }
}
#[doc = "Possible values of the field `RXDP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDPR {
    #[doc = "received data plus upstream port data line"]
    RECEIVED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RXDPR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RXDPR::RECEIVED => true,
            RXDPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDPR {
        match value {
            true => RXDPR::RECEIVED,
            i => RXDPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline]
    pub fn is_received(&self) -> bool {
        *self == RXDPR::RECEIVED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:10 - Frame number"]
    #[inline]
    pub fn fn_(&self) -> FNR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FNR { bits }
    }
    #[doc = "Bits 11:12 - Lost SOF"]
    #[inline]
    pub fn lsof(&self) -> LSOFR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LSOFR { bits }
    }
    #[doc = "Bit 13 - Locked"]
    #[inline]
    pub fn lck(&self) -> LCKR {
        LCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Receive data - line status"]
    #[inline]
    pub fn rxdm(&self) -> RXDMR {
        RXDMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Receive data + line status"]
    #[inline]
    pub fn rxdp(&self) -> RXDPR {
        RXDPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}

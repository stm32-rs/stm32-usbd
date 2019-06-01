#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPMCSR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `LPMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMENR {
    #[doc = "enable the LPM support within the USB device"]
    DISABLED,
    #[doc = "no LPM transactions are handled"]
    ENABLED,
}
impl LPMENR {
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
            LPMENR::DISABLED => false,
            LPMENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPMENR {
        match value {
            false => LPMENR::DISABLED,
            true => LPMENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LPMENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LPMENR::ENABLED
    }
}
#[doc = "Possible values of the field `LPMACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMACKR {
    #[doc = "the valid LPM Token will be NYET"]
    NYET,
    #[doc = "the valid LPM Token will be ACK"]
    ACK,
}
impl LPMACKR {
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
            LPMACKR::NYET => false,
            LPMACKR::ACK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPMACKR {
        match value {
            false => LPMACKR::NYET,
            true => LPMACKR::ACK,
        }
    }
    #[doc = "Checks if the value of the field is `NYET`"]
    #[inline]
    pub fn is_nyet(&self) -> bool {
        *self == LPMACKR::NYET
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline]
    pub fn is_ack(&self) -> bool {
        *self == LPMACKR::ACK
    }
}
#[doc = r" Value of the field"]
pub struct REMWAKER {
    bits: bool,
}
impl REMWAKER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct BESLR {
    bits: u8,
}
impl BESLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `LPMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMENW {
    #[doc = "enable the LPM support within the USB device"]
    DISABLED,
    #[doc = "no LPM transactions are handled"]
    ENABLED,
}
impl LPMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPMENW::DISABLED => false,
            LPMENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPMENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "enable the LPM support within the USB device"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPMENW::DISABLED)
    }
    #[doc = "no LPM transactions are handled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPMENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPMACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMACKW {
    #[doc = "the valid LPM Token will be NYET"]
    NYET,
    #[doc = "the valid LPM Token will be ACK"]
    ACK,
}
impl LPMACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPMACKW::NYET => false,
            LPMACKW::ACK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPMACKW<'a> {
    w: &'a mut W,
}
impl<'a> _LPMACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPMACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the valid LPM Token will be NYET"]
    #[inline]
    pub fn nyet(self) -> &'a mut W {
        self.variant(LPMACKW::NYET)
    }
    #[doc = "the valid LPM Token will be ACK"]
    #[inline]
    pub fn ack(self) -> &'a mut W {
        self.variant(LPMACKW::ACK)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - LPM support enable"]
    #[inline]
    pub fn lpmen(&self) -> LPMENR {
        LPMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - LPM Token acknowledge enable"]
    #[inline]
    pub fn lpmack(&self) -> LPMACKR {
        LPMACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - bRemoteWake value"]
    #[inline]
    pub fn remwake(&self) -> REMWAKER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        REMWAKER { bits }
    }
    #[doc = "Bits 4:7 - BESL value"]
    #[inline]
    pub fn besl(&self) -> BESLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BESLR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - LPM support enable"]
    #[inline]
    pub fn lpmen(&mut self) -> _LPMENW {
        _LPMENW { w: self }
    }
    #[doc = "Bit 1 - LPM Token acknowledge enable"]
    #[inline]
    pub fn lpmack(&mut self) -> _LPMACKW {
        _LPMACKW { w: self }
    }
}

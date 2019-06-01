#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BCDR {
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
#[doc = "Possible values of the field `BCDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCDENR {
    #[doc = "disable the BCD support"]
    DISABLED,
    #[doc = "enable the BCD support within the USB device"]
    ENABLED,
}
impl BCDENR {
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
            BCDENR::DISABLED => false,
            BCDENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BCDENR {
        match value {
            false => BCDENR::DISABLED,
            true => BCDENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == BCDENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BCDENR::ENABLED
    }
}
#[doc = "Possible values of the field `DCDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDENR {
    #[doc = "Data contact detection (DCD) mode disabled"]
    DISABLED,
    #[doc = "Data contact detection (DCD) mode enabled"]
    ENABLED,
}
impl DCDENR {
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
            DCDENR::DISABLED => false,
            DCDENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCDENR {
        match value {
            false => DCDENR::DISABLED,
            true => DCDENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DCDENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DCDENR::ENABLED
    }
}
#[doc = "Possible values of the field `PDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDENR {
    #[doc = "Primary detection (PD) mode disabled"]
    DISABLED,
    #[doc = "Primary detection (PD) mode enabled"]
    ENABLED,
}
impl PDENR {
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
            PDENR::DISABLED => false,
            PDENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDENR {
        match value {
            false => PDENR::DISABLED,
            true => PDENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PDENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PDENR::ENABLED
    }
}
#[doc = "Possible values of the field `SDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDENR {
    #[doc = "Secondary detection (SD) mode disabled"]
    DISABLED,
    #[doc = "Secondary detection (SD) mode enabled"]
    ENABLED,
}
impl SDENR {
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
            SDENR::DISABLED => false,
            SDENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDENR {
        match value {
            false => SDENR::DISABLED,
            true => SDENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SDENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SDENR::ENABLED
    }
}
#[doc = "Possible values of the field `DCDET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDETR {
    #[doc = "data lines contact not detected"]
    NOTDETECTED,
    #[doc = "data lines contact detected"]
    DETECTED,
}
impl DCDETR {
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
            DCDETR::NOTDETECTED => false,
            DCDETR::DETECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCDETR {
        match value {
            false => DCDETR::NOTDETECTED,
            true => DCDETR::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline]
    pub fn is_not_detected(&self) -> bool {
        *self == DCDETR::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline]
    pub fn is_detected(&self) -> bool {
        *self == DCDETR::DETECTED
    }
}
#[doc = "Possible values of the field `PDET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDETR {
    #[doc = "no BCD support detected"]
    NOBCD,
    #[doc = "BCD support detected"]
    BCD,
}
impl PDETR {
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
            PDETR::NOBCD => false,
            PDETR::BCD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDETR {
        match value {
            false => PDETR::NOBCD,
            true => PDETR::BCD,
        }
    }
    #[doc = "Checks if the value of the field is `NOBCD`"]
    #[inline]
    pub fn is_no_bcd(&self) -> bool {
        *self == PDETR::NOBCD
    }
    #[doc = "Checks if the value of the field is `BCD`"]
    #[inline]
    pub fn is_bcd(&self) -> bool {
        *self == PDETR::BCD
    }
}
#[doc = "Possible values of the field `SDET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDETR {
    #[doc = "CDP detected"]
    CDP,
    #[doc = "DCP detected"]
    DCP,
}
impl SDETR {
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
            SDETR::CDP => false,
            SDETR::DCP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDETR {
        match value {
            false => SDETR::CDP,
            true => SDETR::DCP,
        }
    }
    #[doc = "Checks if the value of the field is `CDP`"]
    #[inline]
    pub fn is_cdp(&self) -> bool {
        *self == SDETR::CDP
    }
    #[doc = "Checks if the value of the field is `DCP`"]
    #[inline]
    pub fn is_dcp(&self) -> bool {
        *self == SDETR::DCP
    }
}
#[doc = "Possible values of the field `PS2DET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS2DETR {
    #[doc = "Normal port detected"]
    NORMAL,
    #[doc = "PS2 port or proprietary charger detected"]
    PS2,
}
impl PS2DETR {
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
            PS2DETR::NORMAL => false,
            PS2DETR::PS2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PS2DETR {
        match value {
            false => PS2DETR::NORMAL,
            true => PS2DETR::PS2,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == PS2DETR::NORMAL
    }
    #[doc = "Checks if the value of the field is `PS2`"]
    #[inline]
    pub fn is_ps2(&self) -> bool {
        *self == PS2DETR::PS2
    }
}
#[doc = "Possible values of the field `DPPU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPPUR {
    #[doc = "signalize disconnect to the host when needed by the user software"]
    DISABLED,
    #[doc = "enable the embedded pull-up on the DP line"]
    ENABLED,
}
impl DPPUR {
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
            DPPUR::DISABLED => false,
            DPPUR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPPUR {
        match value {
            false => DPPUR::DISABLED,
            true => DPPUR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DPPUR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DPPUR::ENABLED
    }
}
#[doc = "Values that can be written to the field `BCDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCDENW {
    #[doc = "disable the BCD support"]
    DISABLED,
    #[doc = "enable the BCD support within the USB device"]
    ENABLED,
}
impl BCDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BCDENW::DISABLED => false,
            BCDENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCDENW<'a> {
    w: &'a mut W,
}
impl<'a> _BCDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable the BCD support"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BCDENW::DISABLED)
    }
    #[doc = "enable the BCD support within the USB device"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BCDENW::ENABLED)
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
#[doc = "Values that can be written to the field `DCDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDENW {
    #[doc = "Data contact detection (DCD) mode disabled"]
    DISABLED,
    #[doc = "Data contact detection (DCD) mode enabled"]
    ENABLED,
}
impl DCDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCDENW::DISABLED => false,
            DCDENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCDENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data contact detection (DCD) mode disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCDENW::DISABLED)
    }
    #[doc = "Data contact detection (DCD) mode enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCDENW::ENABLED)
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
#[doc = "Values that can be written to the field `PDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDENW {
    #[doc = "Primary detection (PD) mode disabled"]
    DISABLED,
    #[doc = "Primary detection (PD) mode enabled"]
    ENABLED,
}
impl PDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDENW::DISABLED => false,
            PDENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDENW<'a> {
    w: &'a mut W,
}
impl<'a> _PDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Primary detection (PD) mode disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PDENW::DISABLED)
    }
    #[doc = "Primary detection (PD) mode enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PDENW::ENABLED)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDENW {
    #[doc = "Secondary detection (SD) mode disabled"]
    DISABLED,
    #[doc = "Secondary detection (SD) mode enabled"]
    ENABLED,
}
impl SDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDENW::DISABLED => false,
            SDENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDENW<'a> {
    w: &'a mut W,
}
impl<'a> _SDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Secondary detection (SD) mode disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDENW::DISABLED)
    }
    #[doc = "Secondary detection (SD) mode enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDENW::ENABLED)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DPPU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPPUW {
    #[doc = "signalize disconnect to the host when needed by the user software"]
    DISABLED,
    #[doc = "enable the embedded pull-up on the DP line"]
    ENABLED,
}
impl DPPUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPPUW::DISABLED => false,
            DPPUW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPPUW<'a> {
    w: &'a mut W,
}
impl<'a> _DPPUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPPUW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "signalize disconnect to the host when needed by the user software"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DPPUW::DISABLED)
    }
    #[doc = "enable the embedded pull-up on the DP line"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DPPUW::ENABLED)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Battery charging detector (BCD) enable"]
    #[inline]
    pub fn bcden(&self) -> BCDENR {
        BCDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Data contact detection (DCD) mode enable"]
    #[inline]
    pub fn dcden(&self) -> DCDENR {
        DCDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Primary detection (PD) mode enable"]
    #[inline]
    pub fn pden(&self) -> PDENR {
        PDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Secondary detection (SD) mode enable"]
    #[inline]
    pub fn sden(&self) -> SDENR {
        SDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Data contact detection (DCD) status"]
    #[inline]
    pub fn dcdet(&self) -> DCDETR {
        DCDETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Primary detection (PD) status"]
    #[inline]
    pub fn pdet(&self) -> PDETR {
        PDETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Secondary detection (SD) status"]
    #[inline]
    pub fn sdet(&self) -> SDETR {
        SDETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - DM pull-up detection status"]
    #[inline]
    pub fn ps2det(&self) -> PS2DETR {
        PS2DETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - DP pull-up control"]
    #[inline]
    pub fn dppu(&self) -> DPPUR {
        DPPUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 0 - Battery charging detector (BCD) enable"]
    #[inline]
    pub fn bcden(&mut self) -> _BCDENW {
        _BCDENW { w: self }
    }
    #[doc = "Bit 1 - Data contact detection (DCD) mode enable"]
    #[inline]
    pub fn dcden(&mut self) -> _DCDENW {
        _DCDENW { w: self }
    }
    #[doc = "Bit 2 - Primary detection (PD) mode enable"]
    #[inline]
    pub fn pden(&mut self) -> _PDENW {
        _PDENW { w: self }
    }
    #[doc = "Bit 3 - Secondary detection (SD) mode enable"]
    #[inline]
    pub fn sden(&mut self) -> _SDENW {
        _SDENW { w: self }
    }
    #[doc = "Bit 15 - DP pull-up control"]
    #[inline]
    pub fn dppu(&mut self) -> _DPPUW {
        _DPPUW { w: self }
    }
}

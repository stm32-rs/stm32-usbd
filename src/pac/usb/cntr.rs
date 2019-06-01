#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CNTR {
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
#[doc = "Possible values of the field `FRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRESR {
    #[doc = "Clear USB reset"]
    NORESET,
    #[doc = "Force a reset of the USB peripheral, exactly like a RESET signaling on the USB"]
    RESET,
}
impl FRESR {
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
            FRESR::NORESET => false,
            FRESR::RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRESR {
        match value {
            false => FRESR::NORESET,
            true => FRESR::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORESET`"]
    #[inline]
    pub fn is_no_reset(&self) -> bool {
        *self == FRESR::NORESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == FRESR::RESET
    }
}
#[doc = "Possible values of the field `PDWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDWNR {
    #[doc = "No power down"]
    DISABLED,
    #[doc = "Enter power down mode"]
    ENABLED,
}
impl PDWNR {
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
            PDWNR::DISABLED => false,
            PDWNR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDWNR {
        match value {
            false => PDWNR::DISABLED,
            true => PDWNR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PDWNR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PDWNR::ENABLED
    }
}
#[doc = "Possible values of the field `LPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMODER {
    #[doc = "No low-power mode"]
    DISABLED,
    #[doc = "Enter low-power mode"]
    ENABLED,
}
impl LPMODER {
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
            LPMODER::DISABLED => false,
            LPMODER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPMODER {
        match value {
            false => LPMODER::DISABLED,
            true => LPMODER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LPMODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LPMODER::ENABLED
    }
}
#[doc = "Possible values of the field `FSUSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSUSPR {
    #[doc = "No effect"]
    NOEFFECT,
    #[doc = "Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected"]
    SUSPEND,
}
impl FSUSPR {
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
            FSUSPR::NOEFFECT => false,
            FSUSPR::SUSPEND => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSUSPR {
        match value {
            false => FSUSPR::NOEFFECT,
            true => FSUSPR::SUSPEND,
        }
    }
    #[doc = "Checks if the value of the field is `NOEFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == FSUSPR::NOEFFECT
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline]
    pub fn is_suspend(&self) -> bool {
        *self == FSUSPR::SUSPEND
    }
}
#[doc = "Possible values of the field `RESUME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESUMER {
    #[doc = "Resume requested"]
    REQUESTED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RESUMER {
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
            RESUMER::REQUESTED => true,
            RESUMER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESUMER {
        match value {
            true => RESUMER::REQUESTED,
            i => RESUMER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REQUESTED`"]
    #[inline]
    pub fn is_requested(&self) -> bool {
        *self == RESUMER::REQUESTED
    }
}
#[doc = "Possible values of the field `L1RESUME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L1RESUMER {
    #[doc = "LPM L1 request requested"]
    REQUESTED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl L1RESUMER {
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
            L1RESUMER::REQUESTED => true,
            L1RESUMER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L1RESUMER {
        match value {
            true => L1RESUMER::REQUESTED,
            i => L1RESUMER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REQUESTED`"]
    #[inline]
    pub fn is_requested(&self) -> bool {
        *self == L1RESUMER::REQUESTED
    }
}
#[doc = "Possible values of the field `L1REQM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L1REQMR {
    #[doc = "L1REQ Interrupt disabled"]
    DISABLED,
    #[doc = "L1REQ Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl L1REQMR {
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
            L1REQMR::DISABLED => false,
            L1REQMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L1REQMR {
        match value {
            false => L1REQMR::DISABLED,
            true => L1REQMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == L1REQMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == L1REQMR::ENABLED
    }
}
#[doc = "Possible values of the field `ESOFM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESOFMR {
    #[doc = "ESOF Interrupt disabled"]
    DISABLED,
    #[doc = "ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl ESOFMR {
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
            ESOFMR::DISABLED => false,
            ESOFMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESOFMR {
        match value {
            false => ESOFMR::DISABLED,
            true => ESOFMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ESOFMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ESOFMR::ENABLED
    }
}
#[doc = "Possible values of the field `SOFM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFMR {
    #[doc = "SOF Interrupt disabled"]
    DISABLED,
    #[doc = "SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl SOFMR {
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
            SOFMR::DISABLED => false,
            SOFMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOFMR {
        match value {
            false => SOFMR::DISABLED,
            true => SOFMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SOFMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SOFMR::ENABLED
    }
}
#[doc = "Possible values of the field `RESETM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETMR {
    #[doc = "RESET Interrupt disabled"]
    DISABLED,
    #[doc = "RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl RESETMR {
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
            RESETMR::DISABLED => false,
            RESETMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESETMR {
        match value {
            false => RESETMR::DISABLED,
            true => RESETMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RESETMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RESETMR::ENABLED
    }
}
#[doc = "Possible values of the field `SUSPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPMR {
    #[doc = "Suspend Mode Request SUSP Interrupt disabled"]
    DISABLED,
    #[doc = "SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl SUSPMR {
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
            SUSPMR::DISABLED => false,
            SUSPMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUSPMR {
        match value {
            false => SUSPMR::DISABLED,
            true => SUSPMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SUSPMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SUSPMR::ENABLED
    }
}
#[doc = "Possible values of the field `WKUPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPMR {
    #[doc = "WKUP Interrupt disabled"]
    DISABLED,
    #[doc = "WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl WKUPMR {
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
            WKUPMR::DISABLED => false,
            WKUPMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPMR {
        match value {
            false => WKUPMR::DISABLED,
            true => WKUPMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WKUPMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WKUPMR::ENABLED
    }
}
#[doc = "Possible values of the field `ERRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRMR {
    #[doc = "ERR Interrupt disabled"]
    DISABLED,
    #[doc = "ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl ERRMR {
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
            ERRMR::DISABLED => false,
            ERRMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRMR {
        match value {
            false => ERRMR::DISABLED,
            true => ERRMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ERRMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ERRMR::ENABLED
    }
}
#[doc = "Possible values of the field `PMAOVRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMAOVRMR {
    #[doc = "PMAOVR Interrupt disabled"]
    DISABLED,
    #[doc = "PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl PMAOVRMR {
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
            PMAOVRMR::DISABLED => false,
            PMAOVRMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PMAOVRMR {
        match value {
            false => PMAOVRMR::DISABLED,
            true => PMAOVRMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PMAOVRMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PMAOVRMR::ENABLED
    }
}
#[doc = "Possible values of the field `CTRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRMR {
    #[doc = "Correct Transfer (CTR) Interrupt disabled"]
    DISABLED,
    #[doc = "CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl CTRMR {
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
            CTRMR::DISABLED => false,
            CTRMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTRMR {
        match value {
            false => CTRMR::DISABLED,
            true => CTRMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CTRMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CTRMR::ENABLED
    }
}
#[doc = "Values that can be written to the field `FRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRESW {
    #[doc = "Clear USB reset"]
    NORESET,
    #[doc = "Force a reset of the USB peripheral, exactly like a RESET signaling on the USB"]
    RESET,
}
impl FRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRESW::NORESET => false,
            FRESW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRESW<'a> {
    w: &'a mut W,
}
impl<'a> _FRESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear USB reset"]
    #[inline]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(FRESW::NORESET)
    }
    #[doc = "Force a reset of the USB peripheral, exactly like a RESET signaling on the USB"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(FRESW::RESET)
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
#[doc = "Values that can be written to the field `PDWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDWNW {
    #[doc = "No power down"]
    DISABLED,
    #[doc = "Enter power down mode"]
    ENABLED,
}
impl PDWNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDWNW::DISABLED => false,
            PDWNW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDWNW<'a> {
    w: &'a mut W,
}
impl<'a> _PDWNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDWNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No power down"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PDWNW::DISABLED)
    }
    #[doc = "Enter power down mode"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PDWNW::ENABLED)
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
#[doc = "Values that can be written to the field `LPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMODEW {
    #[doc = "No low-power mode"]
    DISABLED,
    #[doc = "Enter low-power mode"]
    ENABLED,
}
impl LPMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPMODEW::DISABLED => false,
            LPMODEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LPMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No low-power mode"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPMODEW::DISABLED)
    }
    #[doc = "Enter low-power mode"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPMODEW::ENABLED)
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
#[doc = "Values that can be written to the field `FSUSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSUSPW {
    #[doc = "No effect"]
    NOEFFECT,
    #[doc = "Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected"]
    SUSPEND,
}
impl FSUSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSUSPW::NOEFFECT => false,
            FSUSPW::SUSPEND => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _FSUSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSUSPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(FSUSPW::NOEFFECT)
    }
    #[doc = "Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected"]
    #[inline]
    pub fn suspend(self) -> &'a mut W {
        self.variant(FSUSPW::SUSPEND)
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
#[doc = "Values that can be written to the field `RESUME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESUMEW {
    #[doc = "Resume requested"]
    REQUESTED,
}
impl RESUMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESUMEW::REQUESTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESUMEW<'a> {
    w: &'a mut W,
}
impl<'a> _RESUMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESUMEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resume requested"]
    #[inline]
    pub fn requested(self) -> &'a mut W {
        self.variant(RESUMEW::REQUESTED)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `L1RESUME`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L1RESUMEW {
    #[doc = "LPM L1 request requested"]
    REQUESTED,
}
impl L1RESUMEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L1RESUMEW::REQUESTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L1RESUMEW<'a> {
    w: &'a mut W,
}
impl<'a> _L1RESUMEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L1RESUMEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LPM L1 request requested"]
    #[inline]
    pub fn requested(self) -> &'a mut W {
        self.variant(L1RESUMEW::REQUESTED)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `L1REQM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L1REQMW {
    #[doc = "L1REQ Interrupt disabled"]
    DISABLED,
    #[doc = "L1REQ Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl L1REQMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L1REQMW::DISABLED => false,
            L1REQMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L1REQMW<'a> {
    w: &'a mut W,
}
impl<'a> _L1REQMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L1REQMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "L1REQ Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(L1REQMW::DISABLED)
    }
    #[doc = "L1REQ Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(L1REQMW::ENABLED)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ESOFM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESOFMW {
    #[doc = "ESOF Interrupt disabled"]
    DISABLED,
    #[doc = "ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl ESOFMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESOFMW::DISABLED => false,
            ESOFMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESOFMW<'a> {
    w: &'a mut W,
}
impl<'a> _ESOFMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESOFMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ESOF Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ESOFMW::DISABLED)
    }
    #[doc = "ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ESOFMW::ENABLED)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SOFM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFMW {
    #[doc = "SOF Interrupt disabled"]
    DISABLED,
    #[doc = "SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl SOFMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOFMW::DISABLED => false,
            SOFMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOFMW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOFMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SOF Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SOFMW::DISABLED)
    }
    #[doc = "SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SOFMW::ENABLED)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESETM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETMW {
    #[doc = "RESET Interrupt disabled"]
    DISABLED,
    #[doc = "RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl RESETMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESETMW::DISABLED => false,
            RESETMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESETMW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESETMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RESET Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RESETMW::DISABLED)
    }
    #[doc = "RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RESETMW::ENABLED)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SUSPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPMW {
    #[doc = "Suspend Mode Request SUSP Interrupt disabled"]
    DISABLED,
    #[doc = "SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl SUSPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUSPMW::DISABLED => false,
            SUSPMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUSPMW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUSPMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Suspend Mode Request SUSP Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SUSPMW::DISABLED)
    }
    #[doc = "SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SUSPMW::ENABLED)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPMW {
    #[doc = "WKUP Interrupt disabled"]
    DISABLED,
    #[doc = "WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl WKUPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPMW::DISABLED => false,
            WKUPMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPMW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WKUP Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WKUPMW::DISABLED)
    }
    #[doc = "WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WKUPMW::ENABLED)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRMW {
    #[doc = "ERR Interrupt disabled"]
    DISABLED,
    #[doc = "ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl ERRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRMW::DISABLED => false,
            ERRMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRMW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "ERR Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRMW::DISABLED)
    }
    #[doc = "ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRMW::ENABLED)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PMAOVRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMAOVRMW {
    #[doc = "PMAOVR Interrupt disabled"]
    DISABLED,
    #[doc = "PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl PMAOVRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PMAOVRMW::DISABLED => false,
            PMAOVRMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMAOVRMW<'a> {
    w: &'a mut W,
}
impl<'a> _PMAOVRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMAOVRMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PMAOVR Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PMAOVRMW::DISABLED)
    }
    #[doc = "PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PMAOVRMW::ENABLED)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRMW {
    #[doc = "Correct Transfer (CTR) Interrupt disabled"]
    DISABLED,
    #[doc = "CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    ENABLED,
}
impl CTRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTRMW::DISABLED => false,
            CTRMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTRMW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTRMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Correct Transfer (CTR) Interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTRMW::DISABLED)
    }
    #[doc = "CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTRMW::ENABLED)
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
    #[doc = "Bit 0 - Force USB Reset"]
    #[inline]
    pub fn fres(&self) -> FRESR {
        FRESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Power down"]
    #[inline]
    pub fn pdwn(&self) -> PDWNR {
        PDWNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline]
    pub fn lpmode(&self) -> LPMODER {
        LPMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Force suspend"]
    #[inline]
    pub fn fsusp(&self) -> FSUSPR {
        FSUSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Resume request"]
    #[inline]
    pub fn resume(&self) -> RESUMER {
        RESUMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - LPM L1 Resume request"]
    #[inline]
    pub fn l1resume(&self) -> L1RESUMER {
        L1RESUMER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt mask"]
    #[inline]
    pub fn l1reqm(&self) -> L1REQMR {
        L1REQMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline]
    pub fn esofm(&self) -> ESOFMR {
        ESOFMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline]
    pub fn sofm(&self) -> SOFMR {
        SOFMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - USB reset interrupt mask"]
    #[inline]
    pub fn resetm(&self) -> RESETMR {
        RESETMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline]
    pub fn suspm(&self) -> SUSPMR {
        SUSPMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline]
    pub fn wkupm(&self) -> WKUPMR {
        WKUPMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline]
    pub fn errm(&self) -> ERRMR {
        ERRMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline]
    pub fn pmaovrm(&self) -> PMAOVRMR {
        PMAOVRMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline]
    pub fn ctrm(&self) -> CTRMR {
        CTRMR::_from({
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
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Force USB Reset"]
    #[inline]
    pub fn fres(&mut self) -> _FRESW {
        _FRESW { w: self }
    }
    #[doc = "Bit 1 - Power down"]
    #[inline]
    pub fn pdwn(&mut self) -> _PDWNW {
        _PDWNW { w: self }
    }
    #[doc = "Bit 2 - Low-power mode"]
    #[inline]
    pub fn lpmode(&mut self) -> _LPMODEW {
        _LPMODEW { w: self }
    }
    #[doc = "Bit 3 - Force suspend"]
    #[inline]
    pub fn fsusp(&mut self) -> _FSUSPW {
        _FSUSPW { w: self }
    }
    #[doc = "Bit 4 - Resume request"]
    #[inline]
    pub fn resume(&mut self) -> _RESUMEW {
        _RESUMEW { w: self }
    }
    #[doc = "Bit 5 - LPM L1 Resume request"]
    #[inline]
    pub fn l1resume(&mut self) -> _L1RESUMEW {
        _L1RESUMEW { w: self }
    }
    #[doc = "Bit 7 - LPM L1 state request interrupt mask"]
    #[inline]
    pub fn l1reqm(&mut self) -> _L1REQMW {
        _L1REQMW { w: self }
    }
    #[doc = "Bit 8 - Expected start of frame interrupt mask"]
    #[inline]
    pub fn esofm(&mut self) -> _ESOFMW {
        _ESOFMW { w: self }
    }
    #[doc = "Bit 9 - Start of frame interrupt mask"]
    #[inline]
    pub fn sofm(&mut self) -> _SOFMW {
        _SOFMW { w: self }
    }
    #[doc = "Bit 10 - USB reset interrupt mask"]
    #[inline]
    pub fn resetm(&mut self) -> _RESETMW {
        _RESETMW { w: self }
    }
    #[doc = "Bit 11 - Suspend mode interrupt mask"]
    #[inline]
    pub fn suspm(&mut self) -> _SUSPMW {
        _SUSPMW { w: self }
    }
    #[doc = "Bit 12 - Wakeup interrupt mask"]
    #[inline]
    pub fn wkupm(&mut self) -> _WKUPMW {
        _WKUPMW { w: self }
    }
    #[doc = "Bit 13 - Error interrupt mask"]
    #[inline]
    pub fn errm(&mut self) -> _ERRMW {
        _ERRMW { w: self }
    }
    #[doc = "Bit 14 - Packet memory area over / underrun interrupt mask"]
    #[inline]
    pub fn pmaovrm(&mut self) -> _PMAOVRMW {
        _PMAOVRMW { w: self }
    }
    #[doc = "Bit 15 - Correct transfer interrupt mask"]
    #[inline]
    pub fn ctrm(&mut self) -> _CTRMW {
        _CTRMW { w: self }
    }
}

#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EPR {
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
#[doc = r" Value of the field"]
pub struct EAR {
    bits: u8,
}
impl EAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `STAT_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STAT_TXR {
    #[doc = "all transmission requests addressed to this endpoint are ignored"]
    DISABLED,
    #[doc = "the endpoint is stalled and all transmission requests result in a STALL handshake"]
    STALL,
    #[doc = "the endpoint is naked and all transmission requests result in a NAK handshake"]
    NAK,
    #[doc = "this endpoint is enabled for transmission"]
    VALID,
}
impl STAT_TXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STAT_TXR::DISABLED => 0,
            STAT_TXR::STALL => 1,
            STAT_TXR::NAK => 2,
            STAT_TXR::VALID => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STAT_TXR {
        match value {
            0 => STAT_TXR::DISABLED,
            1 => STAT_TXR::STALL,
            2 => STAT_TXR::NAK,
            3 => STAT_TXR::VALID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == STAT_TXR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline]
    pub fn is_stall(&self) -> bool {
        *self == STAT_TXR::STALL
    }
    #[doc = "Checks if the value of the field is `NAK`"]
    #[inline]
    pub fn is_nak(&self) -> bool {
        *self == STAT_TXR::NAK
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == STAT_TXR::VALID
    }
}
#[doc = r" Value of the field"]
pub struct DTOG_TXR {
    bits: bool,
}
impl DTOG_TXR {
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
pub struct CTR_TXR {
    bits: bool,
}
impl CTR_TXR {
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
pub struct EP_KINDR {
    bits: bool,
}
impl EP_KINDR {
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
#[doc = "Possible values of the field `EP_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_TYPER {
    #[doc = "Bulk endpoint"]
    BULK,
    #[doc = "Control endpoint"]
    CONTROL,
    #[doc = "Iso endpoint"]
    ISO,
    #[doc = "Interrupt endpoint"]
    INTERRUPT,
}
impl EP_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EP_TYPER::BULK => 0,
            EP_TYPER::CONTROL => 1,
            EP_TYPER::ISO => 2,
            EP_TYPER::INTERRUPT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EP_TYPER {
        match value {
            0 => EP_TYPER::BULK,
            1 => EP_TYPER::CONTROL,
            2 => EP_TYPER::ISO,
            3 => EP_TYPER::INTERRUPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline]
    pub fn is_bulk(&self) -> bool {
        *self == EP_TYPER::BULK
    }
    #[doc = "Checks if the value of the field is `CONTROL`"]
    #[inline]
    pub fn is_control(&self) -> bool {
        *self == EP_TYPER::CONTROL
    }
    #[doc = "Checks if the value of the field is `ISO`"]
    #[inline]
    pub fn is_iso(&self) -> bool {
        *self == EP_TYPER::ISO
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline]
    pub fn is_interrupt(&self) -> bool {
        *self == EP_TYPER::INTERRUPT
    }
}
#[doc = r" Value of the field"]
pub struct SETUPR {
    bits: bool,
}
impl SETUPR {
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
#[doc = "Possible values of the field `STAT_RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STAT_RXR {
    #[doc = "all reception requests addressed to this endpoint are ignored"]
    DISABLED,
    #[doc = "the endpoint is stalled and all reception requests result in a STALL handshake"]
    STALL,
    #[doc = "the endpoint is naked and all reception requests result in a NAK handshake"]
    NAK,
    #[doc = "this endpoint is enabled for reception"]
    VALID,
}
impl STAT_RXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STAT_RXR::DISABLED => 0,
            STAT_RXR::STALL => 1,
            STAT_RXR::NAK => 2,
            STAT_RXR::VALID => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STAT_RXR {
        match value {
            0 => STAT_RXR::DISABLED,
            1 => STAT_RXR::STALL,
            2 => STAT_RXR::NAK,
            3 => STAT_RXR::VALID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == STAT_RXR::DISABLED
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline]
    pub fn is_stall(&self) -> bool {
        *self == STAT_RXR::STALL
    }
    #[doc = "Checks if the value of the field is `NAK`"]
    #[inline]
    pub fn is_nak(&self) -> bool {
        *self == STAT_RXR::NAK
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == STAT_RXR::VALID
    }
}
#[doc = r" Value of the field"]
pub struct DTOG_RXR {
    bits: bool,
}
impl DTOG_RXR {
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
pub struct CTR_RXR {
    bits: bool,
}
impl CTR_RXR {
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
#[doc = r" Proxy"]
pub struct _EAW<'a> {
    w: &'a mut W,
}
impl<'a> _EAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STAT_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STAT_TXW {
    #[doc = "all transmission requests addressed to this endpoint are ignored"]
    DISABLED,
    #[doc = "the endpoint is stalled and all transmission requests result in a STALL handshake"]
    STALL,
    #[doc = "the endpoint is naked and all transmission requests result in a NAK handshake"]
    NAK,
    #[doc = "this endpoint is enabled for transmission"]
    VALID,
}
impl STAT_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STAT_TXW::DISABLED => 0,
            STAT_TXW::STALL => 1,
            STAT_TXW::NAK => 2,
            STAT_TXW::VALID => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STAT_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _STAT_TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STAT_TXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "all transmission requests addressed to this endpoint are ignored"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STAT_TXW::DISABLED)
    }
    #[doc = "the endpoint is stalled and all transmission requests result in a STALL handshake"]
    #[inline]
    pub fn stall(self) -> &'a mut W {
        self.variant(STAT_TXW::STALL)
    }
    #[doc = "the endpoint is naked and all transmission requests result in a NAK handshake"]
    #[inline]
    pub fn nak(self) -> &'a mut W {
        self.variant(STAT_TXW::NAK)
    }
    #[doc = "this endpoint is enabled for transmission"]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(STAT_TXW::VALID)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTOG_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOG_TXW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTR_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _CTR_TXW<'a> {
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
#[doc = r" Proxy"]
pub struct _EP_KINDW<'a> {
    w: &'a mut W,
}
impl<'a> _EP_KINDW<'a> {
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
#[doc = "Values that can be written to the field `EP_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_TYPEW {
    #[doc = "Bulk endpoint"]
    BULK,
    #[doc = "Control endpoint"]
    CONTROL,
    #[doc = "Iso endpoint"]
    ISO,
    #[doc = "Interrupt endpoint"]
    INTERRUPT,
}
impl EP_TYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EP_TYPEW::BULK => 0,
            EP_TYPEW::CONTROL => 1,
            EP_TYPEW::ISO => 2,
            EP_TYPEW::INTERRUPT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EP_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _EP_TYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EP_TYPEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bulk endpoint"]
    #[inline]
    pub fn bulk(self) -> &'a mut W {
        self.variant(EP_TYPEW::BULK)
    }
    #[doc = "Control endpoint"]
    #[inline]
    pub fn control(self) -> &'a mut W {
        self.variant(EP_TYPEW::CONTROL)
    }
    #[doc = "Iso endpoint"]
    #[inline]
    pub fn iso(self) -> &'a mut W {
        self.variant(EP_TYPEW::ISO)
    }
    #[doc = "Interrupt endpoint"]
    #[inline]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(EP_TYPEW::INTERRUPT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SETUPW<'a> {
    w: &'a mut W,
}
impl<'a> _SETUPW<'a> {
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
#[doc = "Values that can be written to the field `STAT_RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STAT_RXW {
    #[doc = "all reception requests addressed to this endpoint are ignored"]
    DISABLED,
    #[doc = "the endpoint is stalled and all reception requests result in a STALL handshake"]
    STALL,
    #[doc = "the endpoint is naked and all reception requests result in a NAK handshake"]
    NAK,
    #[doc = "this endpoint is enabled for reception"]
    VALID,
}
impl STAT_RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STAT_RXW::DISABLED => 0,
            STAT_RXW::STALL => 1,
            STAT_RXW::NAK => 2,
            STAT_RXW::VALID => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STAT_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _STAT_RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STAT_RXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "all reception requests addressed to this endpoint are ignored"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STAT_RXW::DISABLED)
    }
    #[doc = "the endpoint is stalled and all reception requests result in a STALL handshake"]
    #[inline]
    pub fn stall(self) -> &'a mut W {
        self.variant(STAT_RXW::STALL)
    }
    #[doc = "the endpoint is naked and all reception requests result in a NAK handshake"]
    #[inline]
    pub fn nak(self) -> &'a mut W {
        self.variant(STAT_RXW::NAK)
    }
    #[doc = "this endpoint is enabled for reception"]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(STAT_RXW::VALID)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTOG_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOG_RXW<'a> {
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
#[doc = r" Proxy"]
pub struct _CTR_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _CTR_RXW<'a> {
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
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline]
    pub fn ea(&self) -> EAR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EAR { bits }
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline]
    pub fn stat_tx(&self) -> STAT_TXR {
        STAT_TXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline]
    pub fn dtog_tx(&self) -> DTOG_TXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTOG_TXR { bits }
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline]
    pub fn ctr_tx(&self) -> CTR_TXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTR_TXR { bits }
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline]
    pub fn ep_kind(&self) -> EP_KINDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EP_KINDR { bits }
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline]
    pub fn ep_type(&self) -> EP_TYPER {
        EP_TYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline]
    pub fn setup(&self) -> SETUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETUPR { bits }
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline]
    pub fn stat_rx(&self) -> STAT_RXR {
        STAT_RXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline]
    pub fn dtog_rx(&self) -> DTOG_RXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTOG_RXR { bits }
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline]
    pub fn ctr_rx(&self) -> CTR_RXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTR_RXR { bits }
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
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline]
    pub fn ea(&mut self) -> _EAW {
        _EAW { w: self }
    }
    #[doc = "Bits 4:5 - Status bits, for transmission transfers"]
    #[inline]
    pub fn stat_tx(&mut self) -> _STAT_TXW {
        _STAT_TXW { w: self }
    }
    #[doc = "Bit 6 - Data Toggle, for transmission transfers"]
    #[inline]
    pub fn dtog_tx(&mut self) -> _DTOG_TXW {
        _DTOG_TXW { w: self }
    }
    #[doc = "Bit 7 - Correct Transfer for transmission"]
    #[inline]
    pub fn ctr_tx(&mut self) -> _CTR_TXW {
        _CTR_TXW { w: self }
    }
    #[doc = "Bit 8 - Endpoint kind"]
    #[inline]
    pub fn ep_kind(&mut self) -> _EP_KINDW {
        _EP_KINDW { w: self }
    }
    #[doc = "Bits 9:10 - Endpoint type"]
    #[inline]
    pub fn ep_type(&mut self) -> _EP_TYPEW {
        _EP_TYPEW { w: self }
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline]
    pub fn setup(&mut self) -> _SETUPW {
        _SETUPW { w: self }
    }
    #[doc = "Bits 12:13 - Status bits, for reception transfers"]
    #[inline]
    pub fn stat_rx(&mut self) -> _STAT_RXW {
        _STAT_RXW { w: self }
    }
    #[doc = "Bit 14 - Data Toggle, for reception transfers"]
    #[inline]
    pub fn dtog_rx(&mut self) -> _DTOG_RXW {
        _DTOG_RXW { w: self }
    }
    #[doc = "Bit 15 - Correct transfer for reception"]
    #[inline]
    pub fn ctr_rx(&mut self) -> _CTR_RXW {
        _CTR_RXW { w: self }
    }
}

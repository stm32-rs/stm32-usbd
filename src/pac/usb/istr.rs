#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ISTR {
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
pub struct EP_IDR {
    bits: u8,
}
impl EP_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRR {
    #[doc = "data transmitted by the USB peripheral to the host PC"]
    TO,
    #[doc = "data received by the USB peripheral from the host PC"]
    FROM,
}
impl DIRR {
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
            DIRR::TO => false,
            DIRR::FROM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRR {
        match value {
            false => DIRR::TO,
            true => DIRR::FROM,
        }
    }
    #[doc = "Checks if the value of the field is `TO`"]
    #[inline]
    pub fn is_to(&self) -> bool {
        *self == DIRR::TO
    }
    #[doc = "Checks if the value of the field is `FROM`"]
    #[inline]
    pub fn is_from(&self) -> bool {
        *self == DIRR::FROM
    }
}
#[doc = "Possible values of the field `L1REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L1REQR {
    #[doc = "LPM command to enter the L1 state is successfully received and acknowledged"]
    RECEIVED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl L1REQR {
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
            L1REQR::RECEIVED => true,
            L1REQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> L1REQR {
        match value {
            true => L1REQR::RECEIVED,
            i => L1REQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline]
    pub fn is_received(&self) -> bool {
        *self == L1REQR::RECEIVED
    }
}
#[doc = "Possible values of the field `ESOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESOFR {
    #[doc = "an SOF packet is expected but not received"]
    EXPECTEDSTARTOFFRAME,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ESOFR {
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
            ESOFR::EXPECTEDSTARTOFFRAME => true,
            ESOFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESOFR {
        match value {
            true => ESOFR::EXPECTEDSTARTOFFRAME,
            i => ESOFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXPECTEDSTARTOFFRAME`"]
    #[inline]
    pub fn is_expected_start_of_frame(&self) -> bool {
        *self == ESOFR::EXPECTEDSTARTOFFRAME
    }
}
#[doc = "Possible values of the field `SOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFR {
    #[doc = "beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus"]
    STARTOFFRAME,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SOFR {
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
            SOFR::STARTOFFRAME => true,
            SOFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SOFR {
        match value {
            true => SOFR::STARTOFFRAME,
            i => SOFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STARTOFFRAME`"]
    #[inline]
    pub fn is_start_of_frame(&self) -> bool {
        *self == SOFR::STARTOFFRAME
    }
}
#[doc = "Possible values of the field `RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETR {
    #[doc = "peripheral detects an active USB RESET signal at its inputs"]
    RESET,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RESETR {
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
            RESETR::RESET => true,
            RESETR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESETR {
        match value {
            true => RESETR::RESET,
            i => RESETR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == RESETR::RESET
    }
}
#[doc = "Possible values of the field `SUSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPR {
    #[doc = "no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus"]
    SUSPEND,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SUSPR {
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
            SUSPR::SUSPEND => true,
            SUSPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUSPR {
        match value {
            true => SUSPR::SUSPEND,
            i => SUSPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline]
    pub fn is_suspend(&self) -> bool {
        *self == SUSPR::SUSPEND
    }
}
#[doc = "Possible values of the field `WKUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPR {
    #[doc = "activity is detected that wakes up the USB peripheral"]
    WAKEUP,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WKUPR {
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
            WKUPR::WAKEUP => true,
            WKUPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPR {
        match value {
            true => WKUPR::WAKEUP,
            i => WKUPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WAKEUP`"]
    #[inline]
    pub fn is_wakeup(&self) -> bool {
        *self == WKUPR::WAKEUP
    }
}
#[doc = "Possible values of the field `ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRR {
    #[doc = "One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred"]
    ERROR,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ERRR {
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
            ERRR::ERROR => true,
            ERRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRR {
        match value {
            true => ERRR::ERROR,
            i => ERRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == ERRR::ERROR
    }
}
#[doc = "Possible values of the field `PMAOVR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMAOVRR {
    #[doc = "microcontroller has not been able to respond in time to an USB memory request"]
    OVERRUN,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PMAOVRR {
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
            PMAOVRR::OVERRUN => true,
            PMAOVRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PMAOVRR {
        match value {
            true => PMAOVRR::OVERRUN,
            i => PMAOVRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline]
    pub fn is_overrun(&self) -> bool {
        *self == PMAOVRR::OVERRUN
    }
}
#[doc = "Possible values of the field `CTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRR {
    #[doc = "endpoint has successfully completed a transaction"]
    COMPLETED,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CTRR {
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
            CTRR::COMPLETED => true,
            CTRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTRR {
        match value {
            true => CTRR::COMPLETED,
            i => CTRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline]
    pub fn is_completed(&self) -> bool {
        *self == CTRR::COMPLETED
    }
}
#[doc = "Values that can be written to the field `L1REQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum L1REQW {
    #[doc = "LPM command to enter the L1 state is successfully received and acknowledged"]
    RECEIVED,
}
impl L1REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            L1REQW::RECEIVED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _L1REQW<'a> {
    w: &'a mut W,
}
impl<'a> _L1REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: L1REQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LPM command to enter the L1 state is successfully received and acknowledged"]
    #[inline]
    pub fn received(self) -> &'a mut W {
        self.variant(L1REQW::RECEIVED)
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
#[doc = "Values that can be written to the field `ESOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESOFW {
    #[doc = "an SOF packet is expected but not received"]
    EXPECTEDSTARTOFFRAME,
}
impl ESOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESOFW::EXPECTEDSTARTOFFRAME => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESOFW<'a> {
    w: &'a mut W,
}
impl<'a> _ESOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "an SOF packet is expected but not received"]
    #[inline]
    pub fn expected_start_of_frame(self) -> &'a mut W {
        self.variant(ESOFW::EXPECTEDSTARTOFFRAME)
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
#[doc = "Values that can be written to the field `SOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFW {
    #[doc = "beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus"]
    STARTOFFRAME,
}
impl SOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SOFW::STARTOFFRAME => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SOFW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus"]
    #[inline]
    pub fn start_of_frame(self) -> &'a mut W {
        self.variant(SOFW::STARTOFFRAME)
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
#[doc = "Values that can be written to the field `RESET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETW {
    #[doc = "peripheral detects an active USB RESET signal at its inputs"]
    RESET,
}
impl RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESETW::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "peripheral detects an active USB RESET signal at its inputs"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESETW::RESET)
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
#[doc = "Values that can be written to the field `SUSP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSPW {
    #[doc = "no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus"]
    SUSPEND,
}
impl SUSPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUSPW::SUSPEND => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUSPW<'a> {
    w: &'a mut W,
}
impl<'a> _SUSPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUSPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus"]
    #[inline]
    pub fn suspend(self) -> &'a mut W {
        self.variant(SUSPW::SUSPEND)
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
#[doc = "Values that can be written to the field `WKUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPW {
    #[doc = "activity is detected that wakes up the USB peripheral"]
    WAKEUP,
}
impl WKUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPW::WAKEUP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "activity is detected that wakes up the USB peripheral"]
    #[inline]
    pub fn wakeup(self) -> &'a mut W {
        self.variant(WKUPW::WAKEUP)
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
#[doc = "Values that can be written to the field `ERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRW {
    #[doc = "One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred"]
    ERROR,
}
impl ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRW::ERROR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred"]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(ERRW::ERROR)
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
#[doc = "Values that can be written to the field `PMAOVR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMAOVRW {
    #[doc = "microcontroller has not been able to respond in time to an USB memory request"]
    OVERRUN,
}
impl PMAOVRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PMAOVRW::OVERRUN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PMAOVRW<'a> {
    w: &'a mut W,
}
impl<'a> _PMAOVRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMAOVRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "microcontroller has not been able to respond in time to an USB memory request"]
    #[inline]
    pub fn overrun(self) -> &'a mut W {
        self.variant(PMAOVRW::OVERRUN)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Endpoint Identifier"]
    #[inline]
    pub fn ep_id(&self) -> EP_IDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EP_IDR { bits }
    }
    #[doc = "Bit 4 - Direction of transaction"]
    #[inline]
    pub fn dir(&self) -> DIRR {
        DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - LPM L1 state request"]
    #[inline]
    pub fn l1req(&self) -> L1REQR {
        L1REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Expected start frame"]
    #[inline]
    pub fn esof(&self) -> ESOFR {
        ESOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline]
    pub fn sof(&self) -> SOFR {
        SOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - reset request"]
    #[inline]
    pub fn reset(&self) -> RESETR {
        RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Suspend mode request"]
    #[inline]
    pub fn susp(&self) -> SUSPR {
        SUSPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline]
    pub fn wkup(&self) -> WKUPR {
        WKUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Error"]
    #[inline]
    pub fn err(&self) -> ERRR {
        ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Packet memory area over / underrun"]
    #[inline]
    pub fn pmaovr(&self) -> PMAOVRR {
        PMAOVRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Correct transfer"]
    #[inline]
    pub fn ctr(&self) -> CTRR {
        CTRR::_from({
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
    #[doc = "Bit 7 - LPM L1 state request"]
    #[inline]
    pub fn l1req(&mut self) -> _L1REQW {
        _L1REQW { w: self }
    }
    #[doc = "Bit 8 - Expected start frame"]
    #[inline]
    pub fn esof(&mut self) -> _ESOFW {
        _ESOFW { w: self }
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline]
    pub fn sof(&mut self) -> _SOFW {
        _SOFW { w: self }
    }
    #[doc = "Bit 10 - reset request"]
    #[inline]
    pub fn reset(&mut self) -> _RESETW {
        _RESETW { w: self }
    }
    #[doc = "Bit 11 - Suspend mode request"]
    #[inline]
    pub fn susp(&mut self) -> _SUSPW {
        _SUSPW { w: self }
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline]
    pub fn wkup(&mut self) -> _WKUPW {
        _WKUPW { w: self }
    }
    #[doc = "Bit 13 - Error"]
    #[inline]
    pub fn err(&mut self) -> _ERRW {
        _ERRW { w: self }
    }
    #[doc = "Bit 14 - Packet memory area over / underrun"]
    #[inline]
    pub fn pmaovr(&mut self) -> _PMAOVRW {
        _PMAOVRW { w: self }
    }
}

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    pub epr: [EPR; 8],
    _reserved1: [u8; 32usize],
    #[doc = "0x40 - control register"]
    pub cntr: CNTR,
    #[doc = "0x44 - interrupt status register"]
    pub istr: ISTR,
    #[doc = "0x48 - frame number register"]
    pub fnr: FNR,
    #[doc = "0x4c - device address"]
    pub daddr: DADDR,
    #[doc = "0x50 - Buffer table address"]
    pub btable: BTABLE,
    #[doc = "0x54 - LPM control and status register"]
    pub lpmcsr: LPMCSR,
    #[doc = "0x58 - Battery charging detector"]
    pub bcdr: BCDR,
}
#[doc = "endpoint 0 register\n\nThis register you can [`read`](crate::pac::generic::Reg::read), [`reset`](crate::pac::generic::Reg::reset), [`write`](crate::pac::generic::Reg::write), [`write_with_zero`](crate::pac::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epr](epr) module"]
pub type EPR = crate::pac::generic::Reg<u32, _EPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPR;
#[doc = "`read()` method returns [epr::R](epr::R) reader structure"]
impl crate::pac::generic::Readable for EPR {}
#[doc = "`write(|w| ..)` method takes [epr::W](epr::W) writer structure"]
impl crate::pac::generic::Writable for EPR {}
#[doc = "endpoint 0 register"]
pub mod epr;
#[doc = "control register\n\nThis register you can [`read`](crate::pac::generic::Reg::read), [`reset`](crate::pac::generic::Reg::reset), [`write`](crate::pac::generic::Reg::write), [`write_with_zero`](crate::pac::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr](cntr) module"]
pub type CNTR = crate::pac::generic::Reg<u32, _CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTR;
#[doc = "`read()` method returns [cntr::R](cntr::R) reader structure"]
impl crate::pac::generic::Readable for CNTR {}
#[doc = "`write(|w| ..)` method takes [cntr::W](cntr::W) writer structure"]
impl crate::pac::generic::Writable for CNTR {}
#[doc = "control register"]
pub mod cntr;
#[doc = "interrupt status register\n\nThis register you can [`read`](crate::pac::generic::Reg::read), [`reset`](crate::pac::generic::Reg::reset), [`write`](crate::pac::generic::Reg::write), [`write_with_zero`](crate::pac::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [istr](istr) module"]
pub type ISTR = crate::pac::generic::Reg<u32, _ISTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISTR;
#[doc = "`read()` method returns [istr::R](istr::R) reader structure"]
impl crate::pac::generic::Readable for ISTR {}
#[doc = "`write(|w| ..)` method takes [istr::W](istr::W) writer structure"]
impl crate::pac::generic::Writable for ISTR {}
#[doc = "interrupt status register"]
pub mod istr;
#[doc = "frame number register\n\nThis register you can [`read`](crate::pac::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fnr](fnr) module"]
pub type FNR = crate::pac::generic::Reg<u32, _FNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FNR;
#[doc = "`read()` method returns [fnr::R](fnr::R) reader structure"]
impl crate::pac::generic::Readable for FNR {}
#[doc = "frame number register"]
pub mod fnr;
#[doc = "device address\n\nThis register you can [`read`](crate::pac::generic::Reg::read), [`reset`](crate::pac::generic::Reg::reset), [`write`](crate::pac::generic::Reg::write), [`write_with_zero`](crate::pac::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daddr](daddr) module"]
pub type DADDR = crate::pac::generic::Reg<u32, _DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DADDR;
#[doc = "`read()` method returns [daddr::R](daddr::R) reader structure"]
impl crate::pac::generic::Readable for DADDR {}
#[doc = "`write(|w| ..)` method takes [daddr::W](daddr::W) writer structure"]
impl crate::pac::generic::Writable for DADDR {}
#[doc = "device address"]
pub mod daddr;
#[doc = "Buffer table address\n\nThis register you can [`read`](crate::pac::generic::Reg::read), [`reset`](crate::pac::generic::Reg::reset), [`write`](crate::pac::generic::Reg::write), [`write_with_zero`](crate::pac::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btable](btable) module"]
pub type BTABLE = crate::pac::generic::Reg<u32, _BTABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BTABLE;
#[doc = "`read()` method returns [btable::R](btable::R) reader structure"]
impl crate::pac::generic::Readable for BTABLE {}
#[doc = "`write(|w| ..)` method takes [btable::W](btable::W) writer structure"]
impl crate::pac::generic::Writable for BTABLE {}
#[doc = "Buffer table address"]
pub mod btable;
#[doc = "LPM control and status register\n\nThis register you can [`read`](crate::pac::generic::Reg::read), [`reset`](crate::pac::generic::Reg::reset), [`write`](crate::pac::generic::Reg::write), [`write_with_zero`](crate::pac::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmcsr](lpmcsr) module"]
pub type LPMCSR = crate::pac::generic::Reg<u32, _LPMCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMCSR;
#[doc = "`read()` method returns [lpmcsr::R](lpmcsr::R) reader structure"]
impl crate::pac::generic::Readable for LPMCSR {}
#[doc = "`write(|w| ..)` method takes [lpmcsr::W](lpmcsr::W) writer structure"]
impl crate::pac::generic::Writable for LPMCSR {}
#[doc = "LPM control and status register"]
pub mod lpmcsr;
#[doc = "Battery charging detector\n\nThis register you can [`read`](crate::pac::generic::Reg::read), [`reset`](crate::pac::generic::Reg::reset), [`write`](crate::pac::generic::Reg::write), [`write_with_zero`](crate::pac::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcdr](bcdr) module"]
pub type BCDR = crate::pac::generic::Reg<u32, _BCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCDR;
#[doc = "`read()` method returns [bcdr::R](bcdr::R) reader structure"]
impl crate::pac::generic::Readable for BCDR {}
#[doc = "`write(|w| ..)` method takes [bcdr::W](bcdr::W) writer structure"]
impl crate::pac::generic::Writable for BCDR {}
#[doc = "Battery charging detector"]
pub mod bcdr;

#[doc = "Reader of register FNR"]
pub type R = crate::pac::generic::R<u32, super::FNR>;
#[doc = "Reader of field `FN`"]
pub type FN_R = crate::pac::generic::R<u16, u16>;
#[doc = "Reader of field `LSOF`"]
pub type LSOF_R = crate::pac::generic::R<u8, u8>;
#[doc = "Locked\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCK_A {
    #[doc = "1: the frame timer remains in this state until an USB reset or USB suspend event occurs"]
    LOCKED = 1,
}
impl From<LCK_A> for bool {
    #[inline(always)]
    fn from(variant: LCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LCK`"]
pub type LCK_R = crate::pac::generic::R<bool, LCK_A>;
impl LCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::pac::generic::Variant<bool, LCK_A> {
        use crate::pac::generic::Variant::*;
        match self.bits {
            true => Val(LCK_A::LOCKED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LCK_A::LOCKED
    }
}
#[doc = "Receive data - line status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDM_A {
    #[doc = "1: received data minus upstream port data line"]
    RECEIVED = 1,
}
impl From<RXDM_A> for bool {
    #[inline(always)]
    fn from(variant: RXDM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXDM`"]
pub type RXDM_R = crate::pac::generic::R<bool, RXDM_A>;
impl RXDM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::pac::generic::Variant<bool, RXDM_A> {
        use crate::pac::generic::Variant::*;
        match self.bits {
            true => Val(RXDM_A::RECEIVED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == RXDM_A::RECEIVED
    }
}
#[doc = "Receive data + line status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDP_A {
    #[doc = "1: received data plus upstream port data line"]
    RECEIVED = 1,
}
impl From<RXDP_A> for bool {
    #[inline(always)]
    fn from(variant: RXDP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXDP`"]
pub type RXDP_R = crate::pac::generic::R<bool, RXDP_A>;
impl RXDP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::pac::generic::Variant<bool, RXDP_A> {
        use crate::pac::generic::Variant::*;
        match self.bits {
            true => Val(RXDP_A::RECEIVED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == RXDP_A::RECEIVED
    }
}
impl R {
    #[doc = "Bits 0:10 - Frame number"]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:12 - Lost SOF"]
    #[inline(always)]
    pub fn lsof(&self) -> LSOF_R {
        LSOF_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 13 - Locked"]
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive data - line status"]
    #[inline(always)]
    pub fn rxdm(&self) -> RXDM_R {
        RXDM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive data + line status"]
    #[inline(always)]
    pub fn rxdp(&self) -> RXDP_R {
        RXDP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}

#[doc = "Reader of register BTABLE"]
pub type R = crate::pac::generic::R<u32, super::BTABLE>;
#[doc = "Writer for register BTABLE"]
pub type W = crate::pac::generic::W<u32, super::BTABLE>;
#[doc = "Register BTABLE `reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::BTABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BTABLE`"]
pub type BTABLE_R = crate::pac::generic::R<u16, u16>;
#[doc = "Write proxy for field `BTABLE`"]
pub struct BTABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BTABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 3)) | (((value as u32) & 0x1fff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:15 - Buffer table"]
    #[inline(always)]
    pub fn btable(&self) -> BTABLE_R {
        BTABLE_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Buffer table"]
    #[inline(always)]
    pub fn btable(&mut self) -> BTABLE_W {
        BTABLE_W { w: self }
    }
}

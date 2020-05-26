#[doc = "Reader of register MCTM_CRR"]
pub type R = crate::R<u32, super::MCTM_CRR>;
#[doc = "Writer for register MCTM_CRR"]
pub type W = crate::W<u32, super::MCTM_CRR>;
#[doc = "Register MCTM_CRR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCTM_CRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRV`"]
pub type CRV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRV`"]
pub struct CRV_W<'a> {
    w: &'a mut W,
}
impl<'a> CRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRV"]
    #[inline(always)]
    pub fn crv(&self) -> CRV_R {
        CRV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRV"]
    #[inline(always)]
    pub fn crv(&mut self) -> CRV_W {
        CRV_W { w: self }
    }
}

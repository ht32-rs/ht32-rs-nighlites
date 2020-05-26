#[doc = "Reader of register CSIF_SMPROW"]
pub type R = crate::R<u32, super::CSIF_SMPROW>;
#[doc = "Writer for register CSIF_SMPROW"]
pub type W = crate::W<u32, super::CSIF_SMPROW>;
#[doc = "Register CSIF_SMPROW `reset()`'s with value 0"]
impl crate::ResetValue for super::CSIF_SMPROW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSM`"]
pub type RSM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RSM`"]
pub struct RSM_W<'a> {
    w: &'a mut W,
}
impl<'a> RSM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - RSM"]
    #[inline(always)]
    pub fn rsm(&self) -> RSM_R {
        RSM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - RSM"]
    #[inline(always)]
    pub fn rsm(&mut self) -> RSM_W {
        RSM_W { w: self }
    }
}

#[doc = "Reader of register FMC_SBVT0"]
pub type R = crate::R<u32, super::FMC_SBVT0>;
#[doc = "Writer for register FMC_SBVT0"]
pub type W = crate::W<u32, super::FMC_SBVT0>;
#[doc = "Register FMC_SBVT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FMC_SBVT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SBVT`"]
pub type SBVT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SBVT`"]
pub struct SBVT_W<'a> {
    w: &'a mut W,
}
impl<'a> SBVT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SBVT"]
    #[inline(always)]
    pub fn sbvt(&self) -> SBVT_R {
        SBVT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SBVT"]
    #[inline(always)]
    pub fn sbvt(&mut self) -> SBVT_W {
        SBVT_W { w: self }
    }
}

#[doc = "Reader of register ADC_LTR"]
pub type R = crate::R<u32, super::ADC_LTR>;
#[doc = "Writer for register ADC_LTR"]
pub type W = crate::W<u32, super::ADC_LTR>;
#[doc = "Register ADC_LTR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_LTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADLT`"]
pub type ADLT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADLT`"]
pub struct ADLT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - ADLT"]
    #[inline(always)]
    pub fn adlt(&self) -> ADLT_R {
        ADLT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADLT"]
    #[inline(always)]
    pub fn adlt(&mut self) -> ADLT_W {
        ADLT_W { w: self }
    }
}

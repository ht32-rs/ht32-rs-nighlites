#[doc = "Reader of register ADC_UTR"]
pub type R = crate::R<u32, super::ADC_UTR>;
#[doc = "Writer for register ADC_UTR"]
pub type W = crate::W<u32, super::ADC_UTR>;
#[doc = "Register ADC_UTR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_UTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADUT`"]
pub type ADUT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADUT`"]
pub struct ADUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - ADUT"]
    #[inline(always)]
    pub fn adut(&self) -> ADUT_R {
        ADUT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADUT"]
    #[inline(always)]
    pub fn adut(&mut self) -> ADUT_W {
        ADUT_W { w: self }
    }
}

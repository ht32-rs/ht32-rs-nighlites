#[doc = "Reader of register ADC_STR10"]
pub type R = crate::R<u32, super::ADC_STR10>;
#[doc = "Writer for register ADC_STR10"]
pub type W = crate::W<u32, super::ADC_STR10>;
#[doc = "Register ADC_STR10 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_STR10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADST10`"]
pub type ADST10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADST10`"]
pub struct ADST10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADST10"]
    #[inline(always)]
    pub fn adst10(&self) -> ADST10_R {
        ADST10_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST10"]
    #[inline(always)]
    pub fn adst10(&mut self) -> ADST10_W {
        ADST10_W { w: self }
    }
}

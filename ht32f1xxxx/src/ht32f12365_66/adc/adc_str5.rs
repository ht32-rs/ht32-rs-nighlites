#[doc = "Reader of register ADC_STR5"]
pub type R = crate::R<u32, super::ADC_STR5>;
#[doc = "Writer for register ADC_STR5"]
pub type W = crate::W<u32, super::ADC_STR5>;
#[doc = "Register ADC_STR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_STR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADST5`"]
pub type ADST5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADST5`"]
pub struct ADST5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADST5"]
    #[inline(always)]
    pub fn adst5(&self) -> ADST5_R {
        ADST5_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST5"]
    #[inline(always)]
    pub fn adst5(&mut self) -> ADST5_W {
        ADST5_W { w: self }
    }
}

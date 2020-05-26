#[doc = "Reader of register ADC_STR14"]
pub type R = crate::R<u32, super::ADC_STR14>;
#[doc = "Writer for register ADC_STR14"]
pub type W = crate::W<u32, super::ADC_STR14>;
#[doc = "Register ADC_STR14 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_STR14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADST14`"]
pub type ADST14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADST14`"]
pub struct ADST14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADST14"]
    #[inline(always)]
    pub fn adst14(&self) -> ADST14_R {
        ADST14_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST14"]
    #[inline(always)]
    pub fn adst14(&mut self) -> ADST14_W {
        ADST14_W { w: self }
    }
}

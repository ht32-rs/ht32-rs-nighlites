#[doc = "Reader of register ADC_STR4"]
pub type R = crate::R<u32, super::ADC_STR4>;
#[doc = "Writer for register ADC_STR4"]
pub type W = crate::W<u32, super::ADC_STR4>;
#[doc = "Register ADC_STR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_STR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADST4`"]
pub type ADST4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADST4`"]
pub struct ADST4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADST4"]
    #[inline(always)]
    pub fn adst4(&self) -> ADST4_R {
        ADST4_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST4"]
    #[inline(always)]
    pub fn adst4(&mut self) -> ADST4_W {
        ADST4_W { w: self }
    }
}

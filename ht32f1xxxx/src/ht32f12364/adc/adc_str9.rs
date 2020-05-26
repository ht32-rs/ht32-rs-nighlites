#[doc = "Reader of register ADC_STR9"]
pub type R = crate::R<u32, super::ADC_STR9>;
#[doc = "Writer for register ADC_STR9"]
pub type W = crate::W<u32, super::ADC_STR9>;
#[doc = "Register ADC_STR9 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_STR9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADST9`"]
pub type ADST9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADST9`"]
pub struct ADST9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADST9"]
    #[inline(always)]
    pub fn adst9(&self) -> ADST9_R {
        ADST9_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST9"]
    #[inline(always)]
    pub fn adst9(&mut self) -> ADST9_W {
        ADST9_W { w: self }
    }
}

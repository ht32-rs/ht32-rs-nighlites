#[doc = "Reader of register ADC_STR7"]
pub type R = crate::R<u32, super::ADC_STR7>;
#[doc = "Writer for register ADC_STR7"]
pub type W = crate::W<u32, super::ADC_STR7>;
#[doc = "Register ADC_STR7 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_STR7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADST7`"]
pub type ADST7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADST7`"]
pub struct ADST7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADST7"]
    #[inline(always)]
    pub fn adst7(&self) -> ADST7_R {
        ADST7_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST7"]
    #[inline(always)]
    pub fn adst7(&mut self) -> ADST7_W {
        ADST7_W { w: self }
    }
}

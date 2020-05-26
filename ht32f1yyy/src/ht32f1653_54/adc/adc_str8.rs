#[doc = "Reader of register ADC_STR8"]
pub type R = crate::R<u32, super::ADC_STR8>;
#[doc = "Writer for register ADC_STR8"]
pub type W = crate::W<u32, super::ADC_STR8>;
#[doc = "Register ADC_STR8 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_STR8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADST8`"]
pub type ADST8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADST8`"]
pub struct ADST8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADST8"]
    #[inline(always)]
    pub fn adst8(&self) -> ADST8_R {
        ADST8_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST8"]
    #[inline(always)]
    pub fn adst8(&mut self) -> ADST8_W {
        ADST8_W { w: self }
    }
}

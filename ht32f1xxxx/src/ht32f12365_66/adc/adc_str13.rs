#[doc = "Reader of register ADC_STR13"]
pub type R = crate::R<u32, super::ADC_STR13>;
#[doc = "Writer for register ADC_STR13"]
pub type W = crate::W<u32, super::ADC_STR13>;
#[doc = "Register ADC_STR13 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_STR13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADST13`"]
pub type ADST13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADST13`"]
pub struct ADST13_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADST13"]
    #[inline(always)]
    pub fn adst13(&self) -> ADST13_R {
        ADST13_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST13"]
    #[inline(always)]
    pub fn adst13(&mut self) -> ADST13_W {
        ADST13_W { w: self }
    }
}

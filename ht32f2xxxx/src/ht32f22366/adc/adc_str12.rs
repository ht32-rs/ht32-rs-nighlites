#[doc = "Reader of register ADC_STR12"]
pub type R = crate::R<u32, super::ADC_STR12>;
#[doc = "Writer for register ADC_STR12"]
pub type W = crate::W<u32, super::ADC_STR12>;
#[doc = "Register ADC_STR12 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_STR12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADST12`"]
pub type ADST12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADST12`"]
pub struct ADST12_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADST12"]
    #[inline(always)]
    pub fn adst12(&self) -> ADST12_R {
        ADST12_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST12"]
    #[inline(always)]
    pub fn adst12(&mut self) -> ADST12_W {
        ADST12_W { w: self }
    }
}

#[doc = "Reader of register ADC_STR3"]
pub type R = crate::R<u32, super::ADC_STR3>;
#[doc = "Writer for register ADC_STR3"]
pub type W = crate::W<u32, super::ADC_STR3>;
#[doc = "Register ADC_STR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_STR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADST3`"]
pub type ADST3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADST3`"]
pub struct ADST3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADST3"]
    #[inline(always)]
    pub fn adst3(&self) -> ADST3_R {
        ADST3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST3"]
    #[inline(always)]
    pub fn adst3(&mut self) -> ADST3_W {
        ADST3_W { w: self }
    }
}

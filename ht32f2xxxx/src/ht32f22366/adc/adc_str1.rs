#[doc = "Reader of register ADC_STR1"]
pub type R = crate::R<u32, super::ADC_STR1>;
#[doc = "Writer for register ADC_STR1"]
pub type W = crate::W<u32, super::ADC_STR1>;
#[doc = "Register ADC_STR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_STR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADST1`"]
pub type ADST1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADST1`"]
pub struct ADST1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADST1"]
    #[inline(always)]
    pub fn adst1(&self) -> ADST1_R {
        ADST1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST1"]
    #[inline(always)]
    pub fn adst1(&mut self) -> ADST1_W {
        ADST1_W { w: self }
    }
}

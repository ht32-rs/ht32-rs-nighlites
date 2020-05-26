#[doc = "Reader of register ADC_STR"]
pub type R = crate::R<u32, super::ADC_STR>;
#[doc = "Writer for register ADC_STR"]
pub type W = crate::W<u32, super::ADC_STR>;
#[doc = "Register ADC_STR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_STR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADST`"]
pub type ADST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADST`"]
pub struct ADST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADST"]
    #[inline(always)]
    pub fn adst(&self) -> ADST_R {
        ADST_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST"]
    #[inline(always)]
    pub fn adst(&mut self) -> ADST_W {
        ADST_W { w: self }
    }
}

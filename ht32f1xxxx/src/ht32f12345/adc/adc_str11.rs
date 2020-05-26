#[doc = "Reader of register ADC_STR11"]
pub type R = crate::R<u32, super::ADC_STR11>;
#[doc = "Writer for register ADC_STR11"]
pub type W = crate::W<u32, super::ADC_STR11>;
#[doc = "Register ADC_STR11 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_STR11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADST11`"]
pub type ADST11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADST11`"]
pub struct ADST11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADST11"]
    #[inline(always)]
    pub fn adst11(&self) -> ADST11_R {
        ADST11_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST11"]
    #[inline(always)]
    pub fn adst11(&mut self) -> ADST11_W {
        ADST11_W { w: self }
    }
}

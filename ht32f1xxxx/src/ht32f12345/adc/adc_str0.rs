#[doc = "Reader of register ADC_STR0"]
pub type R = crate::R<u32, super::ADC_STR0>;
#[doc = "Writer for register ADC_STR0"]
pub type W = crate::W<u32, super::ADC_STR0>;
#[doc = "Register ADC_STR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_STR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADST0`"]
pub type ADST0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADST0`"]
pub struct ADST0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADST0"]
    #[inline(always)]
    pub fn adst0(&self) -> ADST0_R {
        ADST0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST0"]
    #[inline(always)]
    pub fn adst0(&mut self) -> ADST0_W {
        ADST0_W { w: self }
    }
}

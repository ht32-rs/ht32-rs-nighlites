#[doc = "Reader of register ADC_RST"]
pub type R = crate::R<u32, super::ADC_RST>;
#[doc = "Writer for register ADC_RST"]
pub type W = crate::W<u32, super::ADC_RST>;
#[doc = "Register ADC_RST `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_RST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADRST`"]
pub type ADRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADRST`"]
pub struct ADRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADRST"]
    #[inline(always)]
    pub fn adrst(&self) -> ADRST_R {
        ADRST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADRST"]
    #[inline(always)]
    pub fn adrst(&mut self) -> ADRST_W {
        ADRST_W { w: self }
    }
}

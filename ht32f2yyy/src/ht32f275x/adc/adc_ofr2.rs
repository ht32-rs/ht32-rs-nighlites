#[doc = "Reader of register ADC_OFR2"]
pub type R = crate::R<u32, super::ADC_OFR2>;
#[doc = "Writer for register ADC_OFR2"]
pub type W = crate::W<u32, super::ADC_OFR2>;
#[doc = "Register ADC_OFR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADOF2`"]
pub type ADOF2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADOF2`"]
pub struct ADOF2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOF2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ADOFE2`"]
pub type ADOFE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADOFE2`"]
pub struct ADOFE2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOFE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - ADOF2"]
    #[inline(always)]
    pub fn adof2(&self) -> ADOF2_R {
        ADOF2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - ADOFE2"]
    #[inline(always)]
    pub fn adofe2(&self) -> ADOFE2_R {
        ADOFE2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF2"]
    #[inline(always)]
    pub fn adof2(&mut self) -> ADOF2_W {
        ADOF2_W { w: self }
    }
    #[doc = "Bit 15 - ADOFE2"]
    #[inline(always)]
    pub fn adofe2(&mut self) -> ADOFE2_W {
        ADOFE2_W { w: self }
    }
}

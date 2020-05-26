#[doc = "Reader of register ADC_OFR3"]
pub type R = crate::R<u32, super::ADC_OFR3>;
#[doc = "Writer for register ADC_OFR3"]
pub type W = crate::W<u32, super::ADC_OFR3>;
#[doc = "Register ADC_OFR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADOF3`"]
pub type ADOF3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADOF3`"]
pub struct ADOF3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOF3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ADOFE3`"]
pub type ADOFE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADOFE3`"]
pub struct ADOFE3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOFE3_W<'a> {
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
    #[doc = "Bits 0:11 - ADOF3"]
    #[inline(always)]
    pub fn adof3(&self) -> ADOF3_R {
        ADOF3_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - ADOFE3"]
    #[inline(always)]
    pub fn adofe3(&self) -> ADOFE3_R {
        ADOFE3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF3"]
    #[inline(always)]
    pub fn adof3(&mut self) -> ADOF3_W {
        ADOF3_W { w: self }
    }
    #[doc = "Bit 15 - ADOFE3"]
    #[inline(always)]
    pub fn adofe3(&mut self) -> ADOFE3_W {
        ADOFE3_W { w: self }
    }
}

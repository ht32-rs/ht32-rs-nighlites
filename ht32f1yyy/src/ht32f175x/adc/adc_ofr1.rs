#[doc = "Reader of register ADC_OFR1"]
pub type R = crate::R<u32, super::ADC_OFR1>;
#[doc = "Writer for register ADC_OFR1"]
pub type W = crate::W<u32, super::ADC_OFR1>;
#[doc = "Register ADC_OFR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADOF1`"]
pub type ADOF1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADOF1`"]
pub struct ADOF1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOF1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ADOFE1`"]
pub type ADOFE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADOFE1`"]
pub struct ADOFE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOFE1_W<'a> {
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
    #[doc = "Bits 0:11 - ADOF1"]
    #[inline(always)]
    pub fn adof1(&self) -> ADOF1_R {
        ADOF1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - ADOFE1"]
    #[inline(always)]
    pub fn adofe1(&self) -> ADOFE1_R {
        ADOFE1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF1"]
    #[inline(always)]
    pub fn adof1(&mut self) -> ADOF1_W {
        ADOF1_W { w: self }
    }
    #[doc = "Bit 15 - ADOFE1"]
    #[inline(always)]
    pub fn adofe1(&mut self) -> ADOFE1_W {
        ADOFE1_W { w: self }
    }
}

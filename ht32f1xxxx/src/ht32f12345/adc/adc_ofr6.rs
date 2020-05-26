#[doc = "Reader of register ADC_OFR6"]
pub type R = crate::R<u32, super::ADC_OFR6>;
#[doc = "Writer for register ADC_OFR6"]
pub type W = crate::W<u32, super::ADC_OFR6>;
#[doc = "Register ADC_OFR6 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADOF6`"]
pub type ADOF6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADOF6`"]
pub struct ADOF6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOF6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ADAL6`"]
pub type ADAL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADAL6`"]
pub struct ADAL6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADAL6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `ADOFE6`"]
pub type ADOFE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADOFE6`"]
pub struct ADOFE6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOFE6_W<'a> {
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
    #[doc = "Bits 0:11 - ADOF6"]
    #[inline(always)]
    pub fn adof6(&self) -> ADOF6_R {
        ADOF6_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL6"]
    #[inline(always)]
    pub fn adal6(&self) -> ADAL6_R {
        ADAL6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADOFE6"]
    #[inline(always)]
    pub fn adofe6(&self) -> ADOFE6_R {
        ADOFE6_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF6"]
    #[inline(always)]
    pub fn adof6(&mut self) -> ADOF6_W {
        ADOF6_W { w: self }
    }
    #[doc = "Bit 14 - ADAL6"]
    #[inline(always)]
    pub fn adal6(&mut self) -> ADAL6_W {
        ADAL6_W { w: self }
    }
    #[doc = "Bit 15 - ADOFE6"]
    #[inline(always)]
    pub fn adofe6(&mut self) -> ADOFE6_W {
        ADOFE6_W { w: self }
    }
}

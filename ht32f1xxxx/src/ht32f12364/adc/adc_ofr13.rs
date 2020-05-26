#[doc = "Reader of register ADC_OFR13"]
pub type R = crate::R<u32, super::ADC_OFR13>;
#[doc = "Writer for register ADC_OFR13"]
pub type W = crate::W<u32, super::ADC_OFR13>;
#[doc = "Register ADC_OFR13 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFR13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADOF13`"]
pub type ADOF13_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADOF13`"]
pub struct ADOF13_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOF13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ADAL13`"]
pub type ADAL13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADAL13`"]
pub struct ADAL13_W<'a> {
    w: &'a mut W,
}
impl<'a> ADAL13_W<'a> {
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
#[doc = "Reader of field `ADOFE13`"]
pub type ADOFE13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADOFE13`"]
pub struct ADOFE13_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOFE13_W<'a> {
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
    #[doc = "Bits 0:11 - ADOF13"]
    #[inline(always)]
    pub fn adof13(&self) -> ADOF13_R {
        ADOF13_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL13"]
    #[inline(always)]
    pub fn adal13(&self) -> ADAL13_R {
        ADAL13_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADOFE13"]
    #[inline(always)]
    pub fn adofe13(&self) -> ADOFE13_R {
        ADOFE13_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF13"]
    #[inline(always)]
    pub fn adof13(&mut self) -> ADOF13_W {
        ADOF13_W { w: self }
    }
    #[doc = "Bit 14 - ADAL13"]
    #[inline(always)]
    pub fn adal13(&mut self) -> ADAL13_W {
        ADAL13_W { w: self }
    }
    #[doc = "Bit 15 - ADOFE13"]
    #[inline(always)]
    pub fn adofe13(&mut self) -> ADOFE13_W {
        ADOFE13_W { w: self }
    }
}

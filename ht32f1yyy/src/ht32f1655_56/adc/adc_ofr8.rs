#[doc = "Reader of register ADC_OFR8"]
pub type R = crate::R<u32, super::ADC_OFR8>;
#[doc = "Writer for register ADC_OFR8"]
pub type W = crate::W<u32, super::ADC_OFR8>;
#[doc = "Register ADC_OFR8 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFR8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADOF8`"]
pub type ADOF8_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADOF8`"]
pub struct ADOF8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOF8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ADAL8`"]
pub type ADAL8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADAL8`"]
pub struct ADAL8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADAL8_W<'a> {
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
#[doc = "Reader of field `ADOFE8`"]
pub type ADOFE8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADOFE8`"]
pub struct ADOFE8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOFE8_W<'a> {
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
    #[doc = "Bits 0:11 - ADOF8"]
    #[inline(always)]
    pub fn adof8(&self) -> ADOF8_R {
        ADOF8_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL8"]
    #[inline(always)]
    pub fn adal8(&self) -> ADAL8_R {
        ADAL8_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADOFE8"]
    #[inline(always)]
    pub fn adofe8(&self) -> ADOFE8_R {
        ADOFE8_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF8"]
    #[inline(always)]
    pub fn adof8(&mut self) -> ADOF8_W {
        ADOF8_W { w: self }
    }
    #[doc = "Bit 14 - ADAL8"]
    #[inline(always)]
    pub fn adal8(&mut self) -> ADAL8_W {
        ADAL8_W { w: self }
    }
    #[doc = "Bit 15 - ADOFE8"]
    #[inline(always)]
    pub fn adofe8(&mut self) -> ADOFE8_W {
        ADOFE8_W { w: self }
    }
}

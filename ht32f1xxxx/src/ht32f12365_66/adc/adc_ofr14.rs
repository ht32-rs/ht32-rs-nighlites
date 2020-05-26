#[doc = "Reader of register ADC_OFR14"]
pub type R = crate::R<u32, super::ADC_OFR14>;
#[doc = "Writer for register ADC_OFR14"]
pub type W = crate::W<u32, super::ADC_OFR14>;
#[doc = "Register ADC_OFR14 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFR14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADOF14`"]
pub type ADOF14_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADOF14`"]
pub struct ADOF14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOF14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ADAL14`"]
pub type ADAL14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADAL14`"]
pub struct ADAL14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADAL14_W<'a> {
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
#[doc = "Reader of field `ADOFE14`"]
pub type ADOFE14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADOFE14`"]
pub struct ADOFE14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOFE14_W<'a> {
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
    #[doc = "Bits 0:11 - ADOF14"]
    #[inline(always)]
    pub fn adof14(&self) -> ADOF14_R {
        ADOF14_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL14"]
    #[inline(always)]
    pub fn adal14(&self) -> ADAL14_R {
        ADAL14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADOFE14"]
    #[inline(always)]
    pub fn adofe14(&self) -> ADOFE14_R {
        ADOFE14_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF14"]
    #[inline(always)]
    pub fn adof14(&mut self) -> ADOF14_W {
        ADOF14_W { w: self }
    }
    #[doc = "Bit 14 - ADAL14"]
    #[inline(always)]
    pub fn adal14(&mut self) -> ADAL14_W {
        ADAL14_W { w: self }
    }
    #[doc = "Bit 15 - ADOFE14"]
    #[inline(always)]
    pub fn adofe14(&mut self) -> ADOFE14_W {
        ADOFE14_W { w: self }
    }
}

#[doc = "Reader of register ADC_OFR0"]
pub type R = crate::R<u32, super::ADC_OFR0>;
#[doc = "Writer for register ADC_OFR0"]
pub type W = crate::W<u32, super::ADC_OFR0>;
#[doc = "Register ADC_OFR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADOF0`"]
pub type ADOF0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADOF0`"]
pub struct ADOF0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOF0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ADAL0`"]
pub type ADAL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADAL0`"]
pub struct ADAL0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADAL0_W<'a> {
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
#[doc = "Reader of field `ADOFE0`"]
pub type ADOFE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADOFE0`"]
pub struct ADOFE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOFE0_W<'a> {
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
    #[doc = "Bits 0:11 - ADOF0"]
    #[inline(always)]
    pub fn adof0(&self) -> ADOF0_R {
        ADOF0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL0"]
    #[inline(always)]
    pub fn adal0(&self) -> ADAL0_R {
        ADAL0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADOFE0"]
    #[inline(always)]
    pub fn adofe0(&self) -> ADOFE0_R {
        ADOFE0_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF0"]
    #[inline(always)]
    pub fn adof0(&mut self) -> ADOF0_W {
        ADOF0_W { w: self }
    }
    #[doc = "Bit 14 - ADAL0"]
    #[inline(always)]
    pub fn adal0(&mut self) -> ADAL0_W {
        ADAL0_W { w: self }
    }
    #[doc = "Bit 15 - ADOFE0"]
    #[inline(always)]
    pub fn adofe0(&mut self) -> ADOFE0_W {
        ADOFE0_W { w: self }
    }
}

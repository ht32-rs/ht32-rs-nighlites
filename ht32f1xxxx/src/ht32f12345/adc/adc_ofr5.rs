#[doc = "Reader of register ADC_OFR5"]
pub type R = crate::R<u32, super::ADC_OFR5>;
#[doc = "Writer for register ADC_OFR5"]
pub type W = crate::W<u32, super::ADC_OFR5>;
#[doc = "Register ADC_OFR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADOF5`"]
pub type ADOF5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADOF5`"]
pub struct ADOF5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOF5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ADAL5`"]
pub type ADAL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADAL5`"]
pub struct ADAL5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADAL5_W<'a> {
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
#[doc = "Reader of field `ADOFE5`"]
pub type ADOFE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADOFE5`"]
pub struct ADOFE5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOFE5_W<'a> {
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
    #[doc = "Bits 0:11 - ADOF5"]
    #[inline(always)]
    pub fn adof5(&self) -> ADOF5_R {
        ADOF5_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL5"]
    #[inline(always)]
    pub fn adal5(&self) -> ADAL5_R {
        ADAL5_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADOFE5"]
    #[inline(always)]
    pub fn adofe5(&self) -> ADOFE5_R {
        ADOFE5_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF5"]
    #[inline(always)]
    pub fn adof5(&mut self) -> ADOF5_W {
        ADOF5_W { w: self }
    }
    #[doc = "Bit 14 - ADAL5"]
    #[inline(always)]
    pub fn adal5(&mut self) -> ADAL5_W {
        ADAL5_W { w: self }
    }
    #[doc = "Bit 15 - ADOFE5"]
    #[inline(always)]
    pub fn adofe5(&mut self) -> ADOFE5_W {
        ADOFE5_W { w: self }
    }
}

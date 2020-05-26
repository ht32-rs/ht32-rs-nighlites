#[doc = "Reader of register ADC_OFR12"]
pub type R = crate::R<u32, super::ADC_OFR12>;
#[doc = "Writer for register ADC_OFR12"]
pub type W = crate::W<u32, super::ADC_OFR12>;
#[doc = "Register ADC_OFR12 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFR12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADOF12`"]
pub type ADOF12_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADOF12`"]
pub struct ADOF12_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOF12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ADAL12`"]
pub type ADAL12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADAL12`"]
pub struct ADAL12_W<'a> {
    w: &'a mut W,
}
impl<'a> ADAL12_W<'a> {
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
#[doc = "Reader of field `ADOFE12`"]
pub type ADOFE12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADOFE12`"]
pub struct ADOFE12_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOFE12_W<'a> {
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
    #[doc = "Bits 0:11 - ADOF12"]
    #[inline(always)]
    pub fn adof12(&self) -> ADOF12_R {
        ADOF12_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL12"]
    #[inline(always)]
    pub fn adal12(&self) -> ADAL12_R {
        ADAL12_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADOFE12"]
    #[inline(always)]
    pub fn adofe12(&self) -> ADOFE12_R {
        ADOFE12_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF12"]
    #[inline(always)]
    pub fn adof12(&mut self) -> ADOF12_W {
        ADOF12_W { w: self }
    }
    #[doc = "Bit 14 - ADAL12"]
    #[inline(always)]
    pub fn adal12(&mut self) -> ADAL12_W {
        ADAL12_W { w: self }
    }
    #[doc = "Bit 15 - ADOFE12"]
    #[inline(always)]
    pub fn adofe12(&mut self) -> ADOFE12_W {
        ADOFE12_W { w: self }
    }
}

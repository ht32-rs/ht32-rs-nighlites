#[doc = "Reader of register ADC_OFR11"]
pub type R = crate::R<u32, super::ADC_OFR11>;
#[doc = "Writer for register ADC_OFR11"]
pub type W = crate::W<u32, super::ADC_OFR11>;
#[doc = "Register ADC_OFR11 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFR11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADOF11`"]
pub type ADOF11_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADOF11`"]
pub struct ADOF11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOF11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ADAL11`"]
pub type ADAL11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADAL11`"]
pub struct ADAL11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADAL11_W<'a> {
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
#[doc = "Reader of field `ADOFE11`"]
pub type ADOFE11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADOFE11`"]
pub struct ADOFE11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOFE11_W<'a> {
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
    #[doc = "Bits 0:11 - ADOF11"]
    #[inline(always)]
    pub fn adof11(&self) -> ADOF11_R {
        ADOF11_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL11"]
    #[inline(always)]
    pub fn adal11(&self) -> ADAL11_R {
        ADAL11_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADOFE11"]
    #[inline(always)]
    pub fn adofe11(&self) -> ADOFE11_R {
        ADOFE11_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF11"]
    #[inline(always)]
    pub fn adof11(&mut self) -> ADOF11_W {
        ADOF11_W { w: self }
    }
    #[doc = "Bit 14 - ADAL11"]
    #[inline(always)]
    pub fn adal11(&mut self) -> ADAL11_W {
        ADAL11_W { w: self }
    }
    #[doc = "Bit 15 - ADOFE11"]
    #[inline(always)]
    pub fn adofe11(&mut self) -> ADOFE11_W {
        ADOFE11_W { w: self }
    }
}

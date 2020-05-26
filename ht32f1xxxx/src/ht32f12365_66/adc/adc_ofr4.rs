#[doc = "Reader of register ADC_OFR4"]
pub type R = crate::R<u32, super::ADC_OFR4>;
#[doc = "Writer for register ADC_OFR4"]
pub type W = crate::W<u32, super::ADC_OFR4>;
#[doc = "Register ADC_OFR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADOF4`"]
pub type ADOF4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADOF4`"]
pub struct ADOF4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOF4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ADAL4`"]
pub type ADAL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADAL4`"]
pub struct ADAL4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADAL4_W<'a> {
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
#[doc = "Reader of field `ADOFE4`"]
pub type ADOFE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADOFE4`"]
pub struct ADOFE4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOFE4_W<'a> {
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
    #[doc = "Bits 0:11 - ADOF4"]
    #[inline(always)]
    pub fn adof4(&self) -> ADOF4_R {
        ADOF4_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL4"]
    #[inline(always)]
    pub fn adal4(&self) -> ADAL4_R {
        ADAL4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADOFE4"]
    #[inline(always)]
    pub fn adofe4(&self) -> ADOFE4_R {
        ADOFE4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF4"]
    #[inline(always)]
    pub fn adof4(&mut self) -> ADOF4_W {
        ADOF4_W { w: self }
    }
    #[doc = "Bit 14 - ADAL4"]
    #[inline(always)]
    pub fn adal4(&mut self) -> ADAL4_W {
        ADAL4_W { w: self }
    }
    #[doc = "Bit 15 - ADOFE4"]
    #[inline(always)]
    pub fn adofe4(&mut self) -> ADOFE4_W {
        ADOFE4_W { w: self }
    }
}

#[doc = "Reader of register ADC_OFR9"]
pub type R = crate::R<u32, super::ADC_OFR9>;
#[doc = "Writer for register ADC_OFR9"]
pub type W = crate::W<u32, super::ADC_OFR9>;
#[doc = "Register ADC_OFR9 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFR9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADOF9`"]
pub type ADOF9_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADOF9`"]
pub struct ADOF9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOF9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ADAL9`"]
pub type ADAL9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADAL9`"]
pub struct ADAL9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADAL9_W<'a> {
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
#[doc = "Reader of field `ADOFE9`"]
pub type ADOFE9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADOFE9`"]
pub struct ADOFE9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOFE9_W<'a> {
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
    #[doc = "Bits 0:11 - ADOF9"]
    #[inline(always)]
    pub fn adof9(&self) -> ADOF9_R {
        ADOF9_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL9"]
    #[inline(always)]
    pub fn adal9(&self) -> ADAL9_R {
        ADAL9_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADOFE9"]
    #[inline(always)]
    pub fn adofe9(&self) -> ADOFE9_R {
        ADOFE9_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF9"]
    #[inline(always)]
    pub fn adof9(&mut self) -> ADOF9_W {
        ADOF9_W { w: self }
    }
    #[doc = "Bit 14 - ADAL9"]
    #[inline(always)]
    pub fn adal9(&mut self) -> ADAL9_W {
        ADAL9_W { w: self }
    }
    #[doc = "Bit 15 - ADOFE9"]
    #[inline(always)]
    pub fn adofe9(&mut self) -> ADOFE9_W {
        ADOFE9_W { w: self }
    }
}

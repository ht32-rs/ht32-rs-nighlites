#[doc = "Reader of register ADC_OFR10"]
pub type R = crate::R<u32, super::ADC_OFR10>;
#[doc = "Writer for register ADC_OFR10"]
pub type W = crate::W<u32, super::ADC_OFR10>;
#[doc = "Register ADC_OFR10 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFR10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADOF10`"]
pub type ADOF10_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADOF10`"]
pub struct ADOF10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOF10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ADAL10`"]
pub type ADAL10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADAL10`"]
pub struct ADAL10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADAL10_W<'a> {
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
#[doc = "Reader of field `ADOFE10`"]
pub type ADOFE10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADOFE10`"]
pub struct ADOFE10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOFE10_W<'a> {
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
    #[doc = "Bits 0:11 - ADOF10"]
    #[inline(always)]
    pub fn adof10(&self) -> ADOF10_R {
        ADOF10_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL10"]
    #[inline(always)]
    pub fn adal10(&self) -> ADAL10_R {
        ADAL10_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADOFE10"]
    #[inline(always)]
    pub fn adofe10(&self) -> ADOFE10_R {
        ADOFE10_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF10"]
    #[inline(always)]
    pub fn adof10(&mut self) -> ADOF10_W {
        ADOF10_W { w: self }
    }
    #[doc = "Bit 14 - ADAL10"]
    #[inline(always)]
    pub fn adal10(&mut self) -> ADAL10_W {
        ADAL10_W { w: self }
    }
    #[doc = "Bit 15 - ADOFE10"]
    #[inline(always)]
    pub fn adofe10(&mut self) -> ADOFE10_W {
        ADOFE10_W { w: self }
    }
}

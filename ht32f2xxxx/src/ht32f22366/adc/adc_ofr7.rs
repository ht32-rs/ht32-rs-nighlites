#[doc = "Reader of register ADC_OFR7"]
pub type R = crate::R<u32, super::ADC_OFR7>;
#[doc = "Writer for register ADC_OFR7"]
pub type W = crate::W<u32, super::ADC_OFR7>;
#[doc = "Register ADC_OFR7 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFR7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADOF7`"]
pub type ADOF7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADOF7`"]
pub struct ADOF7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOF7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ADAL7`"]
pub type ADAL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADAL7`"]
pub struct ADAL7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADAL7_W<'a> {
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
#[doc = "Reader of field `ADOFE7`"]
pub type ADOFE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADOFE7`"]
pub struct ADOFE7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOFE7_W<'a> {
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
    #[doc = "Bits 0:11 - ADOF7"]
    #[inline(always)]
    pub fn adof7(&self) -> ADOF7_R {
        ADOF7_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL7"]
    #[inline(always)]
    pub fn adal7(&self) -> ADAL7_R {
        ADAL7_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADOFE7"]
    #[inline(always)]
    pub fn adofe7(&self) -> ADOFE7_R {
        ADOFE7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF7"]
    #[inline(always)]
    pub fn adof7(&mut self) -> ADOF7_W {
        ADOF7_W { w: self }
    }
    #[doc = "Bit 14 - ADAL7"]
    #[inline(always)]
    pub fn adal7(&mut self) -> ADAL7_W {
        ADAL7_W { w: self }
    }
    #[doc = "Bit 15 - ADOFE7"]
    #[inline(always)]
    pub fn adofe7(&mut self) -> ADOFE7_W {
        ADOFE7_W { w: self }
    }
}

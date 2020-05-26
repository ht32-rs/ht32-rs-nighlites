#[doc = "Reader of register ADC_OFR15"]
pub type R = crate::R<u32, super::ADC_OFR15>;
#[doc = "Writer for register ADC_OFR15"]
pub type W = crate::W<u32, super::ADC_OFR15>;
#[doc = "Register ADC_OFR15 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_OFR15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADOF15`"]
pub type ADOF15_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADOF15`"]
pub struct ADOF15_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOF15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `ADAL15`"]
pub type ADAL15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADAL15`"]
pub struct ADAL15_W<'a> {
    w: &'a mut W,
}
impl<'a> ADAL15_W<'a> {
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
#[doc = "Reader of field `ADOFE15`"]
pub type ADOFE15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADOFE15`"]
pub struct ADOFE15_W<'a> {
    w: &'a mut W,
}
impl<'a> ADOFE15_W<'a> {
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
    #[doc = "Bits 0:11 - ADOF15"]
    #[inline(always)]
    pub fn adof15(&self) -> ADOF15_R {
        ADOF15_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL15"]
    #[inline(always)]
    pub fn adal15(&self) -> ADAL15_R {
        ADAL15_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADOFE15"]
    #[inline(always)]
    pub fn adofe15(&self) -> ADOFE15_R {
        ADOFE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF15"]
    #[inline(always)]
    pub fn adof15(&mut self) -> ADOF15_W {
        ADOF15_W { w: self }
    }
    #[doc = "Bit 14 - ADAL15"]
    #[inline(always)]
    pub fn adal15(&mut self) -> ADAL15_W {
        ADAL15_W { w: self }
    }
    #[doc = "Bit 15 - ADOFE15"]
    #[inline(always)]
    pub fn adofe15(&mut self) -> ADOFE15_W {
        ADOFE15_W { w: self }
    }
}

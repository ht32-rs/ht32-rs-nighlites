#[doc = "Reader of register ADC_TSR"]
pub type R = crate::R<u32, super::ADC_TSR>;
#[doc = "Writer for register ADC_TSR"]
pub type W = crate::W<u32, super::ADC_TSR>;
#[doc = "Register ADC_TSR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_TSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADSC`"]
pub type ADSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADSC`"]
pub struct ADSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSC_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ADEXTIS`"]
pub type ADEXTIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADEXTIS`"]
pub struct ADEXTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEXTIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `GPTMS`"]
pub type GPTMS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPTMS`"]
pub struct GPTMS_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `GPTME`"]
pub type GPTME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPTME`"]
pub struct GPTME_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADSC"]
    #[inline(always)]
    pub fn adsc(&self) -> ADSC_R {
        ADSC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - ADEXTIS"]
    #[inline(always)]
    pub fn adextis(&self) -> ADEXTIS_R {
        ADEXTIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - GPTMS"]
    #[inline(always)]
    pub fn gptms(&self) -> GPTMS_R {
        GPTMS_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - GPTME"]
    #[inline(always)]
    pub fn gptme(&self) -> GPTME_R {
        GPTME_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADSC"]
    #[inline(always)]
    pub fn adsc(&mut self) -> ADSC_W {
        ADSC_W { w: self }
    }
    #[doc = "Bits 8:11 - ADEXTIS"]
    #[inline(always)]
    pub fn adextis(&mut self) -> ADEXTIS_W {
        ADEXTIS_W { w: self }
    }
    #[doc = "Bits 16:18 - GPTMS"]
    #[inline(always)]
    pub fn gptms(&mut self) -> GPTMS_W {
        GPTMS_W { w: self }
    }
    #[doc = "Bits 24:26 - GPTME"]
    #[inline(always)]
    pub fn gptme(&mut self) -> GPTME_W {
        GPTME_W { w: self }
    }
}

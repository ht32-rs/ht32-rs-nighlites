#[doc = "Reader of register ADC_HTSR"]
pub type R = crate::R<u32, super::ADC_HTSR>;
#[doc = "Writer for register ADC_HTSR"]
pub type W = crate::W<u32, super::ADC_HTSR>;
#[doc = "Register ADC_HTSR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_HTSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADHSC`"]
pub type ADHSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADHSC`"]
pub struct ADHSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHSC_W<'a> {
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
#[doc = "Reader of field `ADHEXTIS`"]
pub type ADHEXTIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADHEXTIS`"]
pub struct ADHEXTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHEXTIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `HTMS`"]
pub type HTMS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HTMS`"]
pub struct HTMS_W<'a> {
    w: &'a mut W,
}
impl<'a> HTMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `HBFTMS`"]
pub type HBFTMS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HBFTMS`"]
pub struct HBFTMS_W<'a> {
    w: &'a mut W,
}
impl<'a> HBFTMS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `HTME`"]
pub type HTME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HTME`"]
pub struct HTME_W<'a> {
    w: &'a mut W,
}
impl<'a> HTME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADHSC"]
    #[inline(always)]
    pub fn adhsc(&self) -> ADHSC_R {
        ADHSC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - ADHEXTIS"]
    #[inline(always)]
    pub fn adhextis(&self) -> ADHEXTIS_R {
        ADHEXTIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - HTMS"]
    #[inline(always)]
    pub fn htms(&self) -> HTMS_R {
        HTMS_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - HBFTMS"]
    #[inline(always)]
    pub fn hbftms(&self) -> HBFTMS_R {
        HBFTMS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - HTME"]
    #[inline(always)]
    pub fn htme(&self) -> HTME_R {
        HTME_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADHSC"]
    #[inline(always)]
    pub fn adhsc(&mut self) -> ADHSC_W {
        ADHSC_W { w: self }
    }
    #[doc = "Bits 8:11 - ADHEXTIS"]
    #[inline(always)]
    pub fn adhextis(&mut self) -> ADHEXTIS_W {
        ADHEXTIS_W { w: self }
    }
    #[doc = "Bits 16:18 - HTMS"]
    #[inline(always)]
    pub fn htms(&mut self) -> HTMS_W {
        HTMS_W { w: self }
    }
    #[doc = "Bit 19 - HBFTMS"]
    #[inline(always)]
    pub fn hbftms(&mut self) -> HBFTMS_W {
        HBFTMS_W { w: self }
    }
    #[doc = "Bits 24:26 - HTME"]
    #[inline(always)]
    pub fn htme(&mut self) -> HTME_W {
        HTME_W { w: self }
    }
}

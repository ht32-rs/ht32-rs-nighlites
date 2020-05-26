#[doc = "Reader of register MCTM_TRCFR"]
pub type R = crate::R<u32, super::MCTM_TRCFR>;
#[doc = "Writer for register MCTM_TRCFR"]
pub type W = crate::W<u32, super::MCTM_TRCFR>;
#[doc = "Register MCTM_TRCFR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCTM_TRCFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRSEL`"]
pub type TRSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRSEL`"]
pub struct TRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ETF`"]
pub type ETF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ETF`"]
pub struct ETF_W<'a> {
    w: &'a mut W,
}
impl<'a> ETF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ETIPSC`"]
pub type ETIPSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ETIPSC`"]
pub struct ETIPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETIPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `ETIPOL`"]
pub type ETIPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETIPOL`"]
pub struct ETIPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> ETIPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `ECME`"]
pub type ECME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECME`"]
pub struct ECME_W<'a> {
    w: &'a mut W,
}
impl<'a> ECME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TRSEL"]
    #[inline(always)]
    pub fn trsel(&self) -> TRSEL_R {
        TRSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ETF"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - ETIPSC"]
    #[inline(always)]
    pub fn etipsc(&self) -> ETIPSC_R {
        ETIPSC_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 16 - ETIPOL"]
    #[inline(always)]
    pub fn etipol(&self) -> ETIPOL_R {
        ETIPOL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ECME"]
    #[inline(always)]
    pub fn ecme(&self) -> ECME_R {
        ECME_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TRSEL"]
    #[inline(always)]
    pub fn trsel(&mut self) -> TRSEL_W {
        TRSEL_W { w: self }
    }
    #[doc = "Bits 8:11 - ETF"]
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W {
        ETF_W { w: self }
    }
    #[doc = "Bits 12:13 - ETIPSC"]
    #[inline(always)]
    pub fn etipsc(&mut self) -> ETIPSC_W {
        ETIPSC_W { w: self }
    }
    #[doc = "Bit 16 - ETIPOL"]
    #[inline(always)]
    pub fn etipol(&mut self) -> ETIPOL_W {
        ETIPOL_W { w: self }
    }
    #[doc = "Bit 24 - ECME"]
    #[inline(always)]
    pub fn ecme(&mut self) -> ECME_W {
        ECME_W { w: self }
    }
}

#[doc = "Reader of register I2S_FCR"]
pub type R = crate::R<u32, super::I2S_FCR>;
#[doc = "Writer for register I2S_FCR"]
pub type W = crate::W<u32, super::I2S_FCR>;
#[doc = "Register I2S_FCR `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXTRI`"]
pub type TXTRI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXTRI`"]
pub struct TXTRI_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `RXTRI`"]
pub type RXTRI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXTRI`"]
pub struct RXTRI_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `TXFR`"]
pub type TXFR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFR`"]
pub struct TXFR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RXFR`"]
pub type RXFR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFR`"]
pub struct RXFR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TXTRI"]
    #[inline(always)]
    pub fn txtri(&self) -> TXTRI_R {
        TXTRI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RXTRI"]
    #[inline(always)]
    pub fn rxtri(&self) -> RXTRI_R {
        RXTRI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - TXFR"]
    #[inline(always)]
    pub fn txfr(&self) -> TXFR_R {
        TXFR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RXFR"]
    #[inline(always)]
    pub fn rxfr(&self) -> RXFR_R {
        RXFR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TXTRI"]
    #[inline(always)]
    pub fn txtri(&mut self) -> TXTRI_W {
        TXTRI_W { w: self }
    }
    #[doc = "Bits 4:7 - RXTRI"]
    #[inline(always)]
    pub fn rxtri(&mut self) -> RXTRI_W {
        RXTRI_W { w: self }
    }
    #[doc = "Bit 8 - TXFR"]
    #[inline(always)]
    pub fn txfr(&mut self) -> TXFR_W {
        TXFR_W { w: self }
    }
    #[doc = "Bit 9 - RXFR"]
    #[inline(always)]
    pub fn rxfr(&mut self) -> RXFR_W {
        RXFR_W { w: self }
    }
}

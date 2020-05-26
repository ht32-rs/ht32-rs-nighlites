#[doc = "Reader of register SPI_FCR"]
pub type R = crate::R<u32, super::SPI_FCR>;
#[doc = "Writer for register SPI_FCR"]
pub type W = crate::W<u32, super::SPI_FCR>;
#[doc = "Register SPI_FCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXFTLS`"]
pub type TXFTLS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXFTLS`"]
pub struct TXFTLS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFTLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `RXFTLS`"]
pub type RXFTLS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXFTLS`"]
pub struct RXFTLS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFTLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `TFPR`"]
pub type TFPR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFPR`"]
pub struct TFPR_W<'a> {
    w: &'a mut W,
}
impl<'a> TFPR_W<'a> {
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
#[doc = "Reader of field `RFPR`"]
pub type RFPR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFPR`"]
pub struct RFPR_W<'a> {
    w: &'a mut W,
}
impl<'a> RFPR_W<'a> {
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
#[doc = "Reader of field `FIFOEN`"]
pub type FIFOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFOEN`"]
pub struct FIFOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TXFTLS"]
    #[inline(always)]
    pub fn txftls(&self) -> TXFTLS_R {
        TXFTLS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RXFTLS"]
    #[inline(always)]
    pub fn rxftls(&self) -> RXFTLS_R {
        RXFTLS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - TFPR"]
    #[inline(always)]
    pub fn tfpr(&self) -> TFPR_R {
        TFPR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RFPR"]
    #[inline(always)]
    pub fn rfpr(&self) -> RFPR_R {
        RFPR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FIFOEN"]
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TXFTLS"]
    #[inline(always)]
    pub fn txftls(&mut self) -> TXFTLS_W {
        TXFTLS_W { w: self }
    }
    #[doc = "Bits 4:7 - RXFTLS"]
    #[inline(always)]
    pub fn rxftls(&mut self) -> RXFTLS_W {
        RXFTLS_W { w: self }
    }
    #[doc = "Bit 8 - TFPR"]
    #[inline(always)]
    pub fn tfpr(&mut self) -> TFPR_W {
        TFPR_W { w: self }
    }
    #[doc = "Bit 9 - RFPR"]
    #[inline(always)]
    pub fn rfpr(&mut self) -> RFPR_W {
        RFPR_W { w: self }
    }
    #[doc = "Bit 10 - FIFOEN"]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W {
        FIFOEN_W { w: self }
    }
}

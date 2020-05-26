#[doc = "Reader of register SPI_CR0"]
pub type R = crate::R<u32, super::SPI_CR0>;
#[doc = "Writer for register SPI_CR0"]
pub type W = crate::W<u32, super::SPI_CR0>;
#[doc = "Register SPI_CR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_CR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPIEN`"]
pub type SPIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIEN`"]
pub struct SPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIEN_W<'a> {
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
#[doc = "Reader of field `SELOEN`"]
pub type SELOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELOEN`"]
pub struct SELOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SELOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SSELC`"]
pub type SSELC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSELC`"]
pub struct SSELC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSELC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SPIEN"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - SELOEN"]
    #[inline(always)]
    pub fn seloen(&self) -> SELOEN_R {
        SELOEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SSELC"]
    #[inline(always)]
    pub fn sselc(&self) -> SSELC_R {
        SSELC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPIEN"]
    #[inline(always)]
    pub fn spien(&mut self) -> SPIEN_W {
        SPIEN_W { w: self }
    }
    #[doc = "Bit 3 - SELOEN"]
    #[inline(always)]
    pub fn seloen(&mut self) -> SELOEN_W {
        SELOEN_W { w: self }
    }
    #[doc = "Bit 4 - SSELC"]
    #[inline(always)]
    pub fn sselc(&mut self) -> SSELC_W {
        SSELC_W { w: self }
    }
}

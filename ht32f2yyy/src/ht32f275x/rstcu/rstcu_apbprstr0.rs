#[doc = "Reader of register RSTCU_APBPRSTR0"]
pub type R = crate::R<u32, super::RSTCU_APBPRSTR0>;
#[doc = "Writer for register RSTCU_APBPRSTR0"]
pub type W = crate::W<u32, super::RSTCU_APBPRSTR0>;
#[doc = "Register RSTCU_APBPRSTR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RSTCU_APBPRSTR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C0RST`"]
pub type I2C0RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0RST`"]
pub struct I2C0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0RST_W<'a> {
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
#[doc = "Reader of field `I2C1RST`"]
pub type I2C1RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1RST`"]
pub struct I2C1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SPI0RST`"]
pub type SPI0RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI0RST`"]
pub struct SPI0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0RST_W<'a> {
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
#[doc = "Reader of field `SPI1RST`"]
pub type SPI1RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1RST`"]
pub struct SPI1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `UR0RST`"]
pub type UR0RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UR0RST`"]
pub struct UR0RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UR0RST_W<'a> {
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
#[doc = "Reader of field `UR1RST`"]
pub type UR1RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UR1RST`"]
pub struct UR1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UR1RST_W<'a> {
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
#[doc = "Reader of field `AFIORST`"]
pub type AFIORST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AFIORST`"]
pub struct AFIORST_W<'a> {
    w: &'a mut W,
}
impl<'a> AFIORST_W<'a> {
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
#[doc = "Reader of field `EXTIRST`"]
pub type EXTIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTIRST`"]
pub struct EXTIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIRST_W<'a> {
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
#[doc = "Reader of field `PARST`"]
pub type PARST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARST`"]
pub struct PARST_W<'a> {
    w: &'a mut W,
}
impl<'a> PARST_W<'a> {
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
#[doc = "Reader of field `PBRST`"]
pub type PBRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBRST`"]
pub struct PBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PBRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `PCRST`"]
pub type PCRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCRST`"]
pub struct PCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PCRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `PDRST`"]
pub type PDRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDRST`"]
pub struct PDRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PDRST_W<'a> {
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
#[doc = "Reader of field `PERST`"]
pub type PERST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PERST`"]
pub struct PERST_W<'a> {
    w: &'a mut W,
}
impl<'a> PERST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SCIRST`"]
pub type SCIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCIRST`"]
pub struct SCIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SCIRST_W<'a> {
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
    #[doc = "Bit 0 - I2C0RST"]
    #[inline(always)]
    pub fn i2c0rst(&self) -> I2C0RST_R {
        I2C0RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C1RST"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SPI0RST"]
    #[inline(always)]
    pub fn spi0rst(&self) -> SPI0RST_R {
        SPI0RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UR0RST"]
    #[inline(always)]
    pub fn ur0rst(&self) -> UR0RST_R {
        UR0RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UR1RST"]
    #[inline(always)]
    pub fn ur1rst(&self) -> UR1RST_R {
        UR1RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AFIORST"]
    #[inline(always)]
    pub fn afiorst(&self) -> AFIORST_R {
        AFIORST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EXTIRST"]
    #[inline(always)]
    pub fn extirst(&self) -> EXTIRST_R {
        EXTIRST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PARST"]
    #[inline(always)]
    pub fn parst(&self) -> PARST_R {
        PARST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PBRST"]
    #[inline(always)]
    pub fn pbrst(&self) -> PBRST_R {
        PBRST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PCRST"]
    #[inline(always)]
    pub fn pcrst(&self) -> PCRST_R {
        PCRST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PDRST"]
    #[inline(always)]
    pub fn pdrst(&self) -> PDRST_R {
        PDRST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PERST"]
    #[inline(always)]
    pub fn perst(&self) -> PERST_R {
        PERST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - SCIRST"]
    #[inline(always)]
    pub fn scirst(&self) -> SCIRST_R {
        SCIRST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C0RST"]
    #[inline(always)]
    pub fn i2c0rst(&mut self) -> I2C0RST_W {
        I2C0RST_W { w: self }
    }
    #[doc = "Bit 1 - I2C1RST"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W {
        I2C1RST_W { w: self }
    }
    #[doc = "Bit 4 - SPI0RST"]
    #[inline(always)]
    pub fn spi0rst(&mut self) -> SPI0RST_W {
        SPI0RST_W { w: self }
    }
    #[doc = "Bit 5 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W {
        SPI1RST_W { w: self }
    }
    #[doc = "Bit 8 - UR0RST"]
    #[inline(always)]
    pub fn ur0rst(&mut self) -> UR0RST_W {
        UR0RST_W { w: self }
    }
    #[doc = "Bit 9 - UR1RST"]
    #[inline(always)]
    pub fn ur1rst(&mut self) -> UR1RST_W {
        UR1RST_W { w: self }
    }
    #[doc = "Bit 14 - AFIORST"]
    #[inline(always)]
    pub fn afiorst(&mut self) -> AFIORST_W {
        AFIORST_W { w: self }
    }
    #[doc = "Bit 15 - EXTIRST"]
    #[inline(always)]
    pub fn extirst(&mut self) -> EXTIRST_W {
        EXTIRST_W { w: self }
    }
    #[doc = "Bit 16 - PARST"]
    #[inline(always)]
    pub fn parst(&mut self) -> PARST_W {
        PARST_W { w: self }
    }
    #[doc = "Bit 17 - PBRST"]
    #[inline(always)]
    pub fn pbrst(&mut self) -> PBRST_W {
        PBRST_W { w: self }
    }
    #[doc = "Bit 18 - PCRST"]
    #[inline(always)]
    pub fn pcrst(&mut self) -> PCRST_W {
        PCRST_W { w: self }
    }
    #[doc = "Bit 19 - PDRST"]
    #[inline(always)]
    pub fn pdrst(&mut self) -> PDRST_W {
        PDRST_W { w: self }
    }
    #[doc = "Bit 20 - PERST"]
    #[inline(always)]
    pub fn perst(&mut self) -> PERST_W {
        PERST_W { w: self }
    }
    #[doc = "Bit 24 - SCIRST"]
    #[inline(always)]
    pub fn scirst(&mut self) -> SCIRST_W {
        SCIRST_W { w: self }
    }
}

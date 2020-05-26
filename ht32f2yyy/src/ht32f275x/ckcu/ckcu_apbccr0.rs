#[doc = "Reader of register CKCU_APBCCR0"]
pub type R = crate::R<u32, super::CKCU_APBCCR0>;
#[doc = "Writer for register CKCU_APBCCR0"]
pub type W = crate::W<u32, super::CKCU_APBCCR0>;
#[doc = "Register CKCU_APBCCR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CKCU_APBCCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C0EN`"]
pub type I2C0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0EN`"]
pub struct I2C0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0EN_W<'a> {
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
#[doc = "Reader of field `I2C1EN`"]
pub type I2C1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1EN`"]
pub struct I2C1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1EN_W<'a> {
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
#[doc = "Reader of field `SPI0EN`"]
pub type SPI0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI0EN`"]
pub struct SPI0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0EN_W<'a> {
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
#[doc = "Reader of field `SPI1EN`"]
pub type SPI1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1EN`"]
pub struct SPI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1EN_W<'a> {
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
#[doc = "Reader of field `UR0EN`"]
pub type UR0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UR0EN`"]
pub struct UR0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UR0EN_W<'a> {
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
#[doc = "Reader of field `UR1EN`"]
pub type UR1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UR1EN`"]
pub struct UR1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UR1EN_W<'a> {
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
#[doc = "Reader of field `AFIOEN`"]
pub type AFIOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AFIOEN`"]
pub struct AFIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AFIOEN_W<'a> {
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
#[doc = "Reader of field `EXTIEN`"]
pub type EXTIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTIEN`"]
pub struct EXTIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIEN_W<'a> {
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
#[doc = "Reader of field `PAEN`"]
pub type PAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAEN`"]
pub struct PAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAEN_W<'a> {
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
#[doc = "Reader of field `PBEN`"]
pub type PBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBEN`"]
pub struct PBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PBEN_W<'a> {
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
#[doc = "Reader of field `PCEN`"]
pub type PCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCEN`"]
pub struct PCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCEN_W<'a> {
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
#[doc = "Reader of field `PDEN`"]
pub type PDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN`"]
pub struct PDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_W<'a> {
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
#[doc = "Reader of field `PEEN`"]
pub type PEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEEN`"]
pub struct PEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEEN_W<'a> {
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
#[doc = "Reader of field `SCIEN`"]
pub type SCIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCIEN`"]
pub struct SCIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCIEN_W<'a> {
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
    #[doc = "Bit 0 - I2C0EN"]
    #[inline(always)]
    pub fn i2c0en(&self) -> I2C0EN_R {
        I2C0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C1EN"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SPI0EN"]
    #[inline(always)]
    pub fn spi0en(&self) -> SPI0EN_R {
        SPI0EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI1EN"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UR0EN"]
    #[inline(always)]
    pub fn ur0en(&self) -> UR0EN_R {
        UR0EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UR1EN"]
    #[inline(always)]
    pub fn ur1en(&self) -> UR1EN_R {
        UR1EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AFIOEN"]
    #[inline(always)]
    pub fn afioen(&self) -> AFIOEN_R {
        AFIOEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EXTIEN"]
    #[inline(always)]
    pub fn extien(&self) -> EXTIEN_R {
        EXTIEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PAEN"]
    #[inline(always)]
    pub fn paen(&self) -> PAEN_R {
        PAEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PBEN"]
    #[inline(always)]
    pub fn pben(&self) -> PBEN_R {
        PBEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PCEN"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PDEN"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PEEN"]
    #[inline(always)]
    pub fn peen(&self) -> PEEN_R {
        PEEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - SCIEN"]
    #[inline(always)]
    pub fn scien(&self) -> SCIEN_R {
        SCIEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C0EN"]
    #[inline(always)]
    pub fn i2c0en(&mut self) -> I2C0EN_W {
        I2C0EN_W { w: self }
    }
    #[doc = "Bit 1 - I2C1EN"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W {
        I2C1EN_W { w: self }
    }
    #[doc = "Bit 4 - SPI0EN"]
    #[inline(always)]
    pub fn spi0en(&mut self) -> SPI0EN_W {
        SPI0EN_W { w: self }
    }
    #[doc = "Bit 5 - SPI1EN"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W {
        SPI1EN_W { w: self }
    }
    #[doc = "Bit 8 - UR0EN"]
    #[inline(always)]
    pub fn ur0en(&mut self) -> UR0EN_W {
        UR0EN_W { w: self }
    }
    #[doc = "Bit 9 - UR1EN"]
    #[inline(always)]
    pub fn ur1en(&mut self) -> UR1EN_W {
        UR1EN_W { w: self }
    }
    #[doc = "Bit 14 - AFIOEN"]
    #[inline(always)]
    pub fn afioen(&mut self) -> AFIOEN_W {
        AFIOEN_W { w: self }
    }
    #[doc = "Bit 15 - EXTIEN"]
    #[inline(always)]
    pub fn extien(&mut self) -> EXTIEN_W {
        EXTIEN_W { w: self }
    }
    #[doc = "Bit 16 - PAEN"]
    #[inline(always)]
    pub fn paen(&mut self) -> PAEN_W {
        PAEN_W { w: self }
    }
    #[doc = "Bit 17 - PBEN"]
    #[inline(always)]
    pub fn pben(&mut self) -> PBEN_W {
        PBEN_W { w: self }
    }
    #[doc = "Bit 18 - PCEN"]
    #[inline(always)]
    pub fn pcen(&mut self) -> PCEN_W {
        PCEN_W { w: self }
    }
    #[doc = "Bit 19 - PDEN"]
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W {
        PDEN_W { w: self }
    }
    #[doc = "Bit 20 - PEEN"]
    #[inline(always)]
    pub fn peen(&mut self) -> PEEN_W {
        PEEN_W { w: self }
    }
    #[doc = "Bit 24 - SCIEN"]
    #[inline(always)]
    pub fn scien(&mut self) -> SCIEN_W {
        SCIEN_W { w: self }
    }
}

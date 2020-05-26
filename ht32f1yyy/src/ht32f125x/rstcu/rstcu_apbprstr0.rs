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
#[doc = "Reader of field `I2CRST`"]
pub type I2CRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2CRST`"]
pub struct I2CRST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CRST_W<'a> {
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
#[doc = "Reader of field `SPIRST`"]
pub type SPIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIRST`"]
pub struct SPIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIRST_W<'a> {
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
#[doc = "Reader of field `URRST`"]
pub type URRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URRST`"]
pub struct URRST_W<'a> {
    w: &'a mut W,
}
impl<'a> URRST_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I2CRST"]
    #[inline(always)]
    pub fn i2crst(&self) -> I2CRST_R {
        I2CRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - SPIRST"]
    #[inline(always)]
    pub fn spirst(&self) -> SPIRST_R {
        SPIRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - URRST"]
    #[inline(always)]
    pub fn urrst(&self) -> URRST_R {
        URRST_R::new(((self.bits >> 8) & 0x01) != 0)
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
}
impl W {
    #[doc = "Bit 0 - I2CRST"]
    #[inline(always)]
    pub fn i2crst(&mut self) -> I2CRST_W {
        I2CRST_W { w: self }
    }
    #[doc = "Bit 4 - SPIRST"]
    #[inline(always)]
    pub fn spirst(&mut self) -> SPIRST_W {
        SPIRST_W { w: self }
    }
    #[doc = "Bit 8 - URRST"]
    #[inline(always)]
    pub fn urrst(&mut self) -> URRST_W {
        URRST_W { w: self }
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
}

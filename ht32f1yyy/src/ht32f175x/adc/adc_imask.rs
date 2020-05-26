#[doc = "Reader of register ADC_IMASK"]
pub type R = crate::R<u32, super::ADC_IMASK>;
#[doc = "Writer for register ADC_IMASK"]
pub type W = crate::W<u32, super::ADC_IMASK>;
#[doc = "Register ADC_IMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_IMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADIMASKS`"]
pub type ADIMASKS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIMASKS`"]
pub struct ADIMASKS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIMASKS_W<'a> {
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
#[doc = "Reader of field `ADIMASKG`"]
pub type ADIMASKG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIMASKG`"]
pub struct ADIMASKG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIMASKG_W<'a> {
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
#[doc = "Reader of field `ADIMASKC`"]
pub type ADIMASKC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIMASKC`"]
pub struct ADIMASKC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIMASKC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `ADIMASKL`"]
pub type ADIMASKL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIMASKL`"]
pub struct ADIMASKL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIMASKL_W<'a> {
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
#[doc = "Reader of field `ADIMASKU`"]
pub type ADIMASKU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIMASKU`"]
pub struct ADIMASKU_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIMASKU_W<'a> {
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
#[doc = "Reader of field `ADIMASKO`"]
pub type ADIMASKO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIMASKO`"]
pub struct ADIMASKO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIMASKO_W<'a> {
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
    #[doc = "Bit 0 - ADIMASKS"]
    #[inline(always)]
    pub fn adimasks(&self) -> ADIMASKS_R {
        ADIMASKS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADIMASKG"]
    #[inline(always)]
    pub fn adimaskg(&self) -> ADIMASKG_R {
        ADIMASKG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADIMASKC"]
    #[inline(always)]
    pub fn adimaskc(&self) -> ADIMASKC_R {
        ADIMASKC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADIMASKL"]
    #[inline(always)]
    pub fn adimaskl(&self) -> ADIMASKL_R {
        ADIMASKL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADIMASKU"]
    #[inline(always)]
    pub fn adimasku(&self) -> ADIMASKU_R {
        ADIMASKU_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADIMASKO"]
    #[inline(always)]
    pub fn adimasko(&self) -> ADIMASKO_R {
        ADIMASKO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADIMASKS"]
    #[inline(always)]
    pub fn adimasks(&mut self) -> ADIMASKS_W {
        ADIMASKS_W { w: self }
    }
    #[doc = "Bit 1 - ADIMASKG"]
    #[inline(always)]
    pub fn adimaskg(&mut self) -> ADIMASKG_W {
        ADIMASKG_W { w: self }
    }
    #[doc = "Bit 2 - ADIMASKC"]
    #[inline(always)]
    pub fn adimaskc(&mut self) -> ADIMASKC_W {
        ADIMASKC_W { w: self }
    }
    #[doc = "Bit 16 - ADIMASKL"]
    #[inline(always)]
    pub fn adimaskl(&mut self) -> ADIMASKL_W {
        ADIMASKL_W { w: self }
    }
    #[doc = "Bit 17 - ADIMASKU"]
    #[inline(always)]
    pub fn adimasku(&mut self) -> ADIMASKU_W {
        ADIMASKU_W { w: self }
    }
    #[doc = "Bit 24 - ADIMASKO"]
    #[inline(always)]
    pub fn adimasko(&mut self) -> ADIMASKO_W {
        ADIMASKO_W { w: self }
    }
}

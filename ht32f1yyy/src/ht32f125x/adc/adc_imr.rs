#[doc = "Reader of register ADC_IMR"]
pub type R = crate::R<u32, super::ADC_IMR>;
#[doc = "Writer for register ADC_IMR"]
pub type W = crate::W<u32, super::ADC_IMR>;
#[doc = "Register ADC_IMR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_IMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADIMS`"]
pub type ADIMS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIMS`"]
pub struct ADIMS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIMS_W<'a> {
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
#[doc = "Reader of field `ADIMG`"]
pub type ADIMG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIMG`"]
pub struct ADIMG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIMG_W<'a> {
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
#[doc = "Reader of field `ADIMC`"]
pub type ADIMC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIMC`"]
pub struct ADIMC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIMC_W<'a> {
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
#[doc = "Reader of field `ADIML`"]
pub type ADIML_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIML`"]
pub struct ADIML_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIML_W<'a> {
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
#[doc = "Reader of field `ADIMU`"]
pub type ADIMU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIMU`"]
pub struct ADIMU_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIMU_W<'a> {
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
#[doc = "Reader of field `ADIMO`"]
pub type ADIMO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADIMO`"]
pub struct ADIMO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADIMO_W<'a> {
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
    #[doc = "Bit 0 - ADIMS"]
    #[inline(always)]
    pub fn adims(&self) -> ADIMS_R {
        ADIMS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADIMG"]
    #[inline(always)]
    pub fn adimg(&self) -> ADIMG_R {
        ADIMG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADIMC"]
    #[inline(always)]
    pub fn adimc(&self) -> ADIMC_R {
        ADIMC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADIML"]
    #[inline(always)]
    pub fn adiml(&self) -> ADIML_R {
        ADIML_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADIMU"]
    #[inline(always)]
    pub fn adimu(&self) -> ADIMU_R {
        ADIMU_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADIMO"]
    #[inline(always)]
    pub fn adimo(&self) -> ADIMO_R {
        ADIMO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADIMS"]
    #[inline(always)]
    pub fn adims(&mut self) -> ADIMS_W {
        ADIMS_W { w: self }
    }
    #[doc = "Bit 1 - ADIMG"]
    #[inline(always)]
    pub fn adimg(&mut self) -> ADIMG_W {
        ADIMG_W { w: self }
    }
    #[doc = "Bit 2 - ADIMC"]
    #[inline(always)]
    pub fn adimc(&mut self) -> ADIMC_W {
        ADIMC_W { w: self }
    }
    #[doc = "Bit 16 - ADIML"]
    #[inline(always)]
    pub fn adiml(&mut self) -> ADIML_W {
        ADIML_W { w: self }
    }
    #[doc = "Bit 17 - ADIMU"]
    #[inline(always)]
    pub fn adimu(&mut self) -> ADIMU_W {
        ADIMU_W { w: self }
    }
    #[doc = "Bit 24 - ADIMO"]
    #[inline(always)]
    pub fn adimo(&mut self) -> ADIMO_W {
        ADIMO_W { w: self }
    }
}

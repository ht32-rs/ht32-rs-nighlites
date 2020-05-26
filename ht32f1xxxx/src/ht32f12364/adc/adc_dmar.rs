#[doc = "Reader of register ADC_DMAR"]
pub type R = crate::R<u32, super::ADC_DMAR>;
#[doc = "Writer for register ADC_DMAR"]
pub type W = crate::W<u32, super::ADC_DMAR>;
#[doc = "Register ADC_DMAR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_DMAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDMAS`"]
pub type ADDMAS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDMAS`"]
pub struct ADDMAS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMAS_W<'a> {
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
#[doc = "Reader of field `ADDMAG`"]
pub type ADDMAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDMAG`"]
pub struct ADDMAG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMAG_W<'a> {
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
#[doc = "Reader of field `ADDMAC`"]
pub type ADDMAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDMAC`"]
pub struct ADDMAC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMAC_W<'a> {
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
#[doc = "Reader of field `ADDMAHS`"]
pub type ADDMAHS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDMAHS`"]
pub struct ADDMAHS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMAHS_W<'a> {
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
#[doc = "Reader of field `ADDMAHG`"]
pub type ADDMAHG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDMAHG`"]
pub struct ADDMAHG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMAHG_W<'a> {
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
#[doc = "Reader of field `ADDMAHC`"]
pub type ADDMAHC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDMAHC`"]
pub struct ADDMAHC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMAHC_W<'a> {
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
    #[doc = "Bit 0 - ADDMAS"]
    #[inline(always)]
    pub fn addmas(&self) -> ADDMAS_R {
        ADDMAS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADDMAG"]
    #[inline(always)]
    pub fn addmag(&self) -> ADDMAG_R {
        ADDMAG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADDMAC"]
    #[inline(always)]
    pub fn addmac(&self) -> ADDMAC_R {
        ADDMAC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADDMAHS"]
    #[inline(always)]
    pub fn addmahs(&self) -> ADDMAHS_R {
        ADDMAHS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADDMAHG"]
    #[inline(always)]
    pub fn addmahg(&self) -> ADDMAHG_R {
        ADDMAHG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADDMAHC"]
    #[inline(always)]
    pub fn addmahc(&self) -> ADDMAHC_R {
        ADDMAHC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADDMAS"]
    #[inline(always)]
    pub fn addmas(&mut self) -> ADDMAS_W {
        ADDMAS_W { w: self }
    }
    #[doc = "Bit 1 - ADDMAG"]
    #[inline(always)]
    pub fn addmag(&mut self) -> ADDMAG_W {
        ADDMAG_W { w: self }
    }
    #[doc = "Bit 2 - ADDMAC"]
    #[inline(always)]
    pub fn addmac(&mut self) -> ADDMAC_W {
        ADDMAC_W { w: self }
    }
    #[doc = "Bit 8 - ADDMAHS"]
    #[inline(always)]
    pub fn addmahs(&mut self) -> ADDMAHS_W {
        ADDMAHS_W { w: self }
    }
    #[doc = "Bit 9 - ADDMAHG"]
    #[inline(always)]
    pub fn addmahg(&mut self) -> ADDMAHG_W {
        ADDMAHG_W { w: self }
    }
    #[doc = "Bit 10 - ADDMAHC"]
    #[inline(always)]
    pub fn addmahc(&mut self) -> ADDMAHC_W {
        ADDMAHC_W { w: self }
    }
}

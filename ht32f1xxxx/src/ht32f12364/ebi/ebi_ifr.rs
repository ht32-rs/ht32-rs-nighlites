#[doc = "Reader of register EBI_IFR"]
pub type R = crate::R<u32, super::EBI_IFR>;
#[doc = "Writer for register EBI_IFR"]
pub type W = crate::W<u32, super::EBI_IFR>;
#[doc = "Register EBI_IFR `reset()`'s with value 0"]
impl crate::ResetValue for super::EBI_IFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ARDYTOIF`"]
pub type ARDYTOIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARDYTOIF`"]
pub struct ARDYTOIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ARDYTOIF_W<'a> {
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
#[doc = "Reader of field `ACCERRIF`"]
pub type ACCERRIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCERRIF`"]
pub struct ACCERRIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCERRIF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ARDYTOIF"]
    #[inline(always)]
    pub fn ardytoif(&self) -> ARDYTOIF_R {
        ARDYTOIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ACCERRIF"]
    #[inline(always)]
    pub fn accerrif(&self) -> ACCERRIF_R {
        ACCERRIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARDYTOIF"]
    #[inline(always)]
    pub fn ardytoif(&mut self) -> ARDYTOIF_W {
        ARDYTOIF_W { w: self }
    }
    #[doc = "Bit 1 - ACCERRIF"]
    #[inline(always)]
    pub fn accerrif(&mut self) -> ACCERRIF_W {
        ACCERRIF_W { w: self }
    }
}

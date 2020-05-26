#[doc = "Reader of register RSTCU_AHBPRSTR"]
pub type R = crate::R<u32, super::RSTCU_AHBPRSTR>;
#[doc = "Writer for register RSTCU_AHBPRSTR"]
pub type W = crate::W<u32, super::RSTCU_AHBPRSTR>;
#[doc = "Register RSTCU_AHBPRSTR `reset()`'s with value 0"]
impl crate::ResetValue for super::RSTCU_AHBPRSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMARST`"]
pub type DMARST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMARST`"]
pub struct DMARST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARST_W<'a> {
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
#[doc = "Reader of field `CSIFRST`"]
pub type CSIFRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSIFRST`"]
pub struct CSIFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CSIFRST_W<'a> {
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
    #[doc = "Bit 0 - DMARST"]
    #[inline(always)]
    pub fn dmarst(&self) -> DMARST_R {
        DMARST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSIFRST"]
    #[inline(always)]
    pub fn csifrst(&self) -> CSIFRST_R {
        CSIFRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMARST"]
    #[inline(always)]
    pub fn dmarst(&mut self) -> DMARST_W {
        DMARST_W { w: self }
    }
    #[doc = "Bit 4 - CSIFRST"]
    #[inline(always)]
    pub fn csifrst(&mut self) -> CSIFRST_W {
        CSIFRST_W { w: self }
    }
}

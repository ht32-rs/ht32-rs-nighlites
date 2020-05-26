#[doc = "Reader of register CSIF_SMP"]
pub type R = crate::R<u32, super::CSIF_SMP>;
#[doc = "Writer for register CSIF_SMP"]
pub type W = crate::W<u32, super::CSIF_SMP>;
#[doc = "Register CSIF_SMP `reset()`'s with value 0"]
impl crate::ResetValue for super::CSIF_SMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSML`"]
pub type CSML_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSML`"]
pub struct CSML_W<'a> {
    w: &'a mut W,
}
impl<'a> CSML_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RSML`"]
pub type RSML_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSML`"]
pub struct RSML_W<'a> {
    w: &'a mut W,
}
impl<'a> RSML_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `SMP_EN`"]
pub type SMP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMP_EN`"]
pub struct SMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:12 - CSML"]
    #[inline(always)]
    pub fn csml(&self) -> CSML_R {
        CSML_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - RSML"]
    #[inline(always)]
    pub fn rsml(&self) -> RSML_R {
        RSML_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - SMP_EN"]
    #[inline(always)]
    pub fn smp_en(&self) -> SMP_EN_R {
        SMP_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:12 - CSML"]
    #[inline(always)]
    pub fn csml(&mut self) -> CSML_W {
        CSML_W { w: self }
    }
    #[doc = "Bits 16:20 - RSML"]
    #[inline(always)]
    pub fn rsml(&mut self) -> RSML_W {
        RSML_W { w: self }
    }
    #[doc = "Bit 31 - SMP_EN"]
    #[inline(always)]
    pub fn smp_en(&mut self) -> SMP_EN_W {
        SMP_EN_W { w: self }
    }
}

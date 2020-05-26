#[doc = "Reader of register CSIF_ENR"]
pub type R = crate::R<u32, super::CSIF_ENR>;
#[doc = "Writer for register CSIF_ENR"]
pub type W = crate::W<u32, super::CSIF_ENR>;
#[doc = "Register CSIF_ENR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSIF_ENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSIF_EN`"]
pub type CSIF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSIF_EN`"]
pub struct CSIF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSIF_EN_W<'a> {
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
    #[doc = "Bit 31 - CSIF_EN"]
    #[inline(always)]
    pub fn csif_en(&self) -> CSIF_EN_R {
        CSIF_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - CSIF_EN"]
    #[inline(always)]
    pub fn csif_en(&mut self) -> CSIF_EN_W {
        CSIF_EN_W { w: self }
    }
}

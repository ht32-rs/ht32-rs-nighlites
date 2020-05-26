#[doc = "Reader of register CKCU_CKST"]
pub type R = crate::R<u32, super::CKCU_CKST>;
#[doc = "Writer for register CKCU_CKST"]
pub type W = crate::W<u32, super::CKCU_CKST>;
#[doc = "Register CKCU_CKST `reset()`'s with value 0"]
impl crate::ResetValue for super::CKCU_CKST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLLST`"]
pub type PLLST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLST`"]
pub struct PLLST_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLST_W<'a> {
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
#[doc = "Reader of field `HSEST`"]
pub type HSEST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSEST`"]
pub struct HSEST_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `HSIST`"]
pub type HSIST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSIST`"]
pub struct HSIST_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - PLLST"]
    #[inline(always)]
    pub fn pllst(&self) -> PLLST_R {
        PLLST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - HSEST"]
    #[inline(always)]
    pub fn hsest(&self) -> HSEST_R {
        HSEST_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - HSIST"]
    #[inline(always)]
    pub fn hsist(&self) -> HSIST_R {
        HSIST_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - PLLST"]
    #[inline(always)]
    pub fn pllst(&mut self) -> PLLST_W {
        PLLST_W { w: self }
    }
    #[doc = "Bits 16:17 - HSEST"]
    #[inline(always)]
    pub fn hsest(&mut self) -> HSEST_W {
        HSEST_W { w: self }
    }
    #[doc = "Bits 24:26 - HSIST"]
    #[inline(always)]
    pub fn hsist(&mut self) -> HSIST_W {
        HSIST_W { w: self }
    }
}

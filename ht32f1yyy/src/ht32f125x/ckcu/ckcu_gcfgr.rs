#[doc = "Reader of register CKCU_GCFGR"]
pub type R = crate::R<u32, super::CKCU_GCFGR>;
#[doc = "Writer for register CKCU_GCFGR"]
pub type W = crate::W<u32, super::CKCU_GCFGR>;
#[doc = "Register CKCU_GCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::CKCU_GCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CKOUTSRC`"]
pub type CKOUTSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKOUTSRC`"]
pub struct CKOUTSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOUTSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `WDTSRC`"]
pub type WDTSRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTSRC`"]
pub struct WDTSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTSRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PLLSRC`"]
pub type PLLSRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSRC`"]
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
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
#[doc = "Reader of field `URPRE`"]
pub type URPRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `URPRE`"]
pub struct URPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> URPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `LPMOD`"]
pub type LPMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPMOD`"]
pub struct LPMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - CKOUTSRC"]
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CKOUTSRC_R {
        CKOUTSRC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - WDTSRC"]
    #[inline(always)]
    pub fn wdtsrc(&self) -> WDTSRC_R {
        WDTSRC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLLSRC"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - URPRE"]
    #[inline(always)]
    pub fn urpre(&self) -> URPRE_R {
        URPRE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 29:31 - LPMOD"]
    #[inline(always)]
    pub fn lpmod(&self) -> LPMOD_R {
        LPMOD_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CKOUTSRC"]
    #[inline(always)]
    pub fn ckoutsrc(&mut self) -> CKOUTSRC_W {
        CKOUTSRC_W { w: self }
    }
    #[doc = "Bit 3 - WDTSRC"]
    #[inline(always)]
    pub fn wdtsrc(&mut self) -> WDTSRC_W {
        WDTSRC_W { w: self }
    }
    #[doc = "Bit 8 - PLLSRC"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W { w: self }
    }
    #[doc = "Bits 20:21 - URPRE"]
    #[inline(always)]
    pub fn urpre(&mut self) -> URPRE_W {
        URPRE_W { w: self }
    }
    #[doc = "Bits 29:31 - LPMOD"]
    #[inline(always)]
    pub fn lpmod(&mut self) -> LPMOD_W {
        LPMOD_W { w: self }
    }
}

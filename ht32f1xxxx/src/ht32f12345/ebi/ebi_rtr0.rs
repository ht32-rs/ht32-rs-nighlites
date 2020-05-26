#[doc = "Reader of register EBI_RTR0"]
pub type R = crate::R<u32, super::EBI_RTR0>;
#[doc = "Writer for register EBI_RTR0"]
pub type W = crate::W<u32, super::EBI_RTR0>;
#[doc = "Register EBI_RTR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::EBI_RTR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RDSETUP`"]
pub type RDSETUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDSETUP`"]
pub struct RDSETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDSETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `RDSTRB`"]
pub type RDSTRB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDSTRB`"]
pub struct RDSTRB_W<'a> {
    w: &'a mut W,
}
impl<'a> RDSTRB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RDHOLD`"]
pub type RDHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDHOLD`"]
pub struct RDHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RDHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PGEN`"]
pub type PGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGEN`"]
pub struct PGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PGEN_W<'a> {
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
    #[doc = "Bits 0:3 - RDSETUP"]
    #[inline(always)]
    pub fn rdsetup(&self) -> RDSETUP_R {
        RDSETUP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - RDSTRB"]
    #[inline(always)]
    pub fn rdstrb(&self) -> RDSTRB_R {
        RDSTRB_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - RDHOLD"]
    #[inline(always)]
    pub fn rdhold(&self) -> RDHOLD_R {
        RDHOLD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - PGEN"]
    #[inline(always)]
    pub fn pgen(&self) -> PGEN_R {
        PGEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - RDSETUP"]
    #[inline(always)]
    pub fn rdsetup(&mut self) -> RDSETUP_W {
        RDSETUP_W { w: self }
    }
    #[doc = "Bits 8:13 - RDSTRB"]
    #[inline(always)]
    pub fn rdstrb(&mut self) -> RDSTRB_W {
        RDSTRB_W { w: self }
    }
    #[doc = "Bits 16:19 - RDHOLD"]
    #[inline(always)]
    pub fn rdhold(&mut self) -> RDHOLD_W {
        RDHOLD_W { w: self }
    }
    #[doc = "Bit 24 - PGEN"]
    #[inline(always)]
    pub fn pgen(&mut self) -> PGEN_W {
        PGEN_W { w: self }
    }
}

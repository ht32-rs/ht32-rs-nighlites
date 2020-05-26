#[doc = "Reader of register EBI_PCR"]
pub type R = crate::R<u32, super::EBI_PCR>;
#[doc = "Writer for register EBI_PCR"]
pub type W = crate::W<u32, super::EBI_PCR>;
#[doc = "Register EBI_PCR `reset()`'s with value 0"]
impl crate::ResetValue for super::EBI_PCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PAGELEN`"]
pub type PAGELEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAGELEN`"]
pub struct PAGELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGELEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `INCHIT`"]
pub type INCHIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCHIT`"]
pub struct INCHIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INCHIT_W<'a> {
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
#[doc = "Reader of field `RDPG`"]
pub type RDPG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDPG`"]
pub struct RDPG_W<'a> {
    w: &'a mut W,
}
impl<'a> RDPG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PAGEOPEN`"]
pub type PAGEOPEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAGEOPEN`"]
pub struct PAGEOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGEOPEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PAGELEN"]
    #[inline(always)]
    pub fn pagelen(&self) -> PAGELEN_R {
        PAGELEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - INCHIT"]
    #[inline(always)]
    pub fn inchit(&self) -> INCHIT_R {
        INCHIT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - RDPG"]
    #[inline(always)]
    pub fn rdpg(&self) -> RDPG_R {
        RDPG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - PAGEOPEN"]
    #[inline(always)]
    pub fn pageopen(&self) -> PAGEOPEN_R {
        PAGEOPEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PAGELEN"]
    #[inline(always)]
    pub fn pagelen(&mut self) -> PAGELEN_W {
        PAGELEN_W { w: self }
    }
    #[doc = "Bit 4 - INCHIT"]
    #[inline(always)]
    pub fn inchit(&mut self) -> INCHIT_W {
        INCHIT_W { w: self }
    }
    #[doc = "Bits 8:11 - RDPG"]
    #[inline(always)]
    pub fn rdpg(&mut self) -> RDPG_W {
        RDPG_W { w: self }
    }
    #[doc = "Bits 16:23 - PAGEOPEN"]
    #[inline(always)]
    pub fn pageopen(&mut self) -> PAGEOPEN_W {
        PAGEOPEN_W { w: self }
    }
}

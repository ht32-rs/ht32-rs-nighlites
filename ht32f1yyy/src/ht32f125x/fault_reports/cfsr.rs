#[doc = "Reader of register CFSR"]
pub type R = crate::R<u32, super::CFSR>;
#[doc = "Writer for register CFSR"]
pub type W = crate::W<u32, super::CFSR>;
#[doc = "Register CFSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CFSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MFSR`"]
pub type MFSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MFSR`"]
pub struct MFSR_W<'a> {
    w: &'a mut W,
}
impl<'a> MFSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `BFSR`"]
pub type BFSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BFSR`"]
pub struct BFSR_W<'a> {
    w: &'a mut W,
}
impl<'a> BFSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `UFSR`"]
pub type UFSR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UFSR`"]
pub struct UFSR_W<'a> {
    w: &'a mut W,
}
impl<'a> UFSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - MFSR"]
    #[inline(always)]
    pub fn mfsr(&self) -> MFSR_R {
        MFSR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - BFSR"]
    #[inline(always)]
    pub fn bfsr(&self) -> BFSR_R {
        BFSR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - UFSR"]
    #[inline(always)]
    pub fn ufsr(&self) -> UFSR_R {
        UFSR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - MFSR"]
    #[inline(always)]
    pub fn mfsr(&mut self) -> MFSR_W {
        MFSR_W { w: self }
    }
    #[doc = "Bits 8:15 - BFSR"]
    #[inline(always)]
    pub fn bfsr(&mut self) -> BFSR_W {
        BFSR_W { w: self }
    }
    #[doc = "Bits 16:31 - UFSR"]
    #[inline(always)]
    pub fn ufsr(&mut self) -> UFSR_W {
        UFSR_W { w: self }
    }
}

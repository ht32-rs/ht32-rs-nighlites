#[doc = "Reader of register AFIO_GPDCFGHR"]
pub type R = crate::R<u32, super::AFIO_GPDCFGHR>;
#[doc = "Writer for register AFIO_GPDCFGHR"]
pub type W = crate::W<u32, super::AFIO_GPDCFGHR>;
#[doc = "Register AFIO_GPDCFGHR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFIO_GPDCFGHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDCFG8`"]
pub type PDCFG8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDCFG8`"]
pub struct PDCFG8_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PDCFG9`"]
pub type PDCFG9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDCFG9`"]
pub struct PDCFG9_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PDCFG10`"]
pub type PDCFG10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDCFG10`"]
pub struct PDCFG10_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PDCFG11`"]
pub type PDCFG11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDCFG11`"]
pub struct PDCFG11_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `PDCFG12`"]
pub type PDCFG12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDCFG12`"]
pub struct PDCFG12_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PDCFG13`"]
pub type PDCFG13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDCFG13`"]
pub struct PDCFG13_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PDCFG14`"]
pub type PDCFG14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDCFG14`"]
pub struct PDCFG14_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `PDCFG15`"]
pub type PDCFG15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDCFG15`"]
pub struct PDCFG15_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PDCFG8"]
    #[inline(always)]
    pub fn pdcfg8(&self) -> PDCFG8_R {
        PDCFG8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PDCFG9"]
    #[inline(always)]
    pub fn pdcfg9(&self) -> PDCFG9_R {
        PDCFG9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PDCFG10"]
    #[inline(always)]
    pub fn pdcfg10(&self) -> PDCFG10_R {
        PDCFG10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PDCFG11"]
    #[inline(always)]
    pub fn pdcfg11(&self) -> PDCFG11_R {
        PDCFG11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PDCFG12"]
    #[inline(always)]
    pub fn pdcfg12(&self) -> PDCFG12_R {
        PDCFG12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PDCFG13"]
    #[inline(always)]
    pub fn pdcfg13(&self) -> PDCFG13_R {
        PDCFG13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PDCFG14"]
    #[inline(always)]
    pub fn pdcfg14(&self) -> PDCFG14_R {
        PDCFG14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PDCFG15"]
    #[inline(always)]
    pub fn pdcfg15(&self) -> PDCFG15_R {
        PDCFG15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDCFG8"]
    #[inline(always)]
    pub fn pdcfg8(&mut self) -> PDCFG8_W {
        PDCFG8_W { w: self }
    }
    #[doc = "Bits 4:7 - PDCFG9"]
    #[inline(always)]
    pub fn pdcfg9(&mut self) -> PDCFG9_W {
        PDCFG9_W { w: self }
    }
    #[doc = "Bits 8:11 - PDCFG10"]
    #[inline(always)]
    pub fn pdcfg10(&mut self) -> PDCFG10_W {
        PDCFG10_W { w: self }
    }
    #[doc = "Bits 12:15 - PDCFG11"]
    #[inline(always)]
    pub fn pdcfg11(&mut self) -> PDCFG11_W {
        PDCFG11_W { w: self }
    }
    #[doc = "Bits 16:19 - PDCFG12"]
    #[inline(always)]
    pub fn pdcfg12(&mut self) -> PDCFG12_W {
        PDCFG12_W { w: self }
    }
    #[doc = "Bits 20:23 - PDCFG13"]
    #[inline(always)]
    pub fn pdcfg13(&mut self) -> PDCFG13_W {
        PDCFG13_W { w: self }
    }
    #[doc = "Bits 24:27 - PDCFG14"]
    #[inline(always)]
    pub fn pdcfg14(&mut self) -> PDCFG14_W {
        PDCFG14_W { w: self }
    }
    #[doc = "Bits 28:31 - PDCFG15"]
    #[inline(always)]
    pub fn pdcfg15(&mut self) -> PDCFG15_W {
        PDCFG15_W { w: self }
    }
}

#[doc = "Reader of register AFIO_GPACFGHR"]
pub type R = crate::R<u32, super::AFIO_GPACFGHR>;
#[doc = "Writer for register AFIO_GPACFGHR"]
pub type W = crate::W<u32, super::AFIO_GPACFGHR>;
#[doc = "Register AFIO_GPACFGHR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFIO_GPACFGHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PACFG8`"]
pub type PACFG8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACFG8`"]
pub struct PACFG8_W<'a> {
    w: &'a mut W,
}
impl<'a> PACFG8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PACFG9`"]
pub type PACFG9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACFG9`"]
pub struct PACFG9_W<'a> {
    w: &'a mut W,
}
impl<'a> PACFG9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PACFG10`"]
pub type PACFG10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACFG10`"]
pub struct PACFG10_W<'a> {
    w: &'a mut W,
}
impl<'a> PACFG10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PACFG11`"]
pub type PACFG11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACFG11`"]
pub struct PACFG11_W<'a> {
    w: &'a mut W,
}
impl<'a> PACFG11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `PACFG12`"]
pub type PACFG12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACFG12`"]
pub struct PACFG12_W<'a> {
    w: &'a mut W,
}
impl<'a> PACFG12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PACFG13`"]
pub type PACFG13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACFG13`"]
pub struct PACFG13_W<'a> {
    w: &'a mut W,
}
impl<'a> PACFG13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PACFG14`"]
pub type PACFG14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACFG14`"]
pub struct PACFG14_W<'a> {
    w: &'a mut W,
}
impl<'a> PACFG14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `PACFG15`"]
pub type PACFG15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACFG15`"]
pub struct PACFG15_W<'a> {
    w: &'a mut W,
}
impl<'a> PACFG15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PACFG8"]
    #[inline(always)]
    pub fn pacfg8(&self) -> PACFG8_R {
        PACFG8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PACFG9"]
    #[inline(always)]
    pub fn pacfg9(&self) -> PACFG9_R {
        PACFG9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PACFG10"]
    #[inline(always)]
    pub fn pacfg10(&self) -> PACFG10_R {
        PACFG10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PACFG11"]
    #[inline(always)]
    pub fn pacfg11(&self) -> PACFG11_R {
        PACFG11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PACFG12"]
    #[inline(always)]
    pub fn pacfg12(&self) -> PACFG12_R {
        PACFG12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PACFG13"]
    #[inline(always)]
    pub fn pacfg13(&self) -> PACFG13_R {
        PACFG13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PACFG14"]
    #[inline(always)]
    pub fn pacfg14(&self) -> PACFG14_R {
        PACFG14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PACFG15"]
    #[inline(always)]
    pub fn pacfg15(&self) -> PACFG15_R {
        PACFG15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PACFG8"]
    #[inline(always)]
    pub fn pacfg8(&mut self) -> PACFG8_W {
        PACFG8_W { w: self }
    }
    #[doc = "Bits 4:7 - PACFG9"]
    #[inline(always)]
    pub fn pacfg9(&mut self) -> PACFG9_W {
        PACFG9_W { w: self }
    }
    #[doc = "Bits 8:11 - PACFG10"]
    #[inline(always)]
    pub fn pacfg10(&mut self) -> PACFG10_W {
        PACFG10_W { w: self }
    }
    #[doc = "Bits 12:15 - PACFG11"]
    #[inline(always)]
    pub fn pacfg11(&mut self) -> PACFG11_W {
        PACFG11_W { w: self }
    }
    #[doc = "Bits 16:19 - PACFG12"]
    #[inline(always)]
    pub fn pacfg12(&mut self) -> PACFG12_W {
        PACFG12_W { w: self }
    }
    #[doc = "Bits 20:23 - PACFG13"]
    #[inline(always)]
    pub fn pacfg13(&mut self) -> PACFG13_W {
        PACFG13_W { w: self }
    }
    #[doc = "Bits 24:27 - PACFG14"]
    #[inline(always)]
    pub fn pacfg14(&mut self) -> PACFG14_W {
        PACFG14_W { w: self }
    }
    #[doc = "Bits 28:31 - PACFG15"]
    #[inline(always)]
    pub fn pacfg15(&mut self) -> PACFG15_W {
        PACFG15_W { w: self }
    }
}

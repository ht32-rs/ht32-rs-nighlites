#[doc = "Reader of register AFIO_GPECFGHR"]
pub type R = crate::R<u32, super::AFIO_GPECFGHR>;
#[doc = "Writer for register AFIO_GPECFGHR"]
pub type W = crate::W<u32, super::AFIO_GPECFGHR>;
#[doc = "Register AFIO_GPECFGHR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFIO_GPECFGHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PECFG8`"]
pub type PECFG8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PECFG8`"]
pub struct PECFG8_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFG8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PECFG9`"]
pub type PECFG9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PECFG9`"]
pub struct PECFG9_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFG9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PECFG10`"]
pub type PECFG10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PECFG10`"]
pub struct PECFG10_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFG10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PECFG11`"]
pub type PECFG11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PECFG11`"]
pub struct PECFG11_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFG11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `PECFG12`"]
pub type PECFG12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PECFG12`"]
pub struct PECFG12_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFG12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PECFG13`"]
pub type PECFG13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PECFG13`"]
pub struct PECFG13_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFG13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PECFG14`"]
pub type PECFG14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PECFG14`"]
pub struct PECFG14_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFG14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `PECFG15`"]
pub type PECFG15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PECFG15`"]
pub struct PECFG15_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFG15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PECFG8"]
    #[inline(always)]
    pub fn pecfg8(&self) -> PECFG8_R {
        PECFG8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PECFG9"]
    #[inline(always)]
    pub fn pecfg9(&self) -> PECFG9_R {
        PECFG9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PECFG10"]
    #[inline(always)]
    pub fn pecfg10(&self) -> PECFG10_R {
        PECFG10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PECFG11"]
    #[inline(always)]
    pub fn pecfg11(&self) -> PECFG11_R {
        PECFG11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PECFG12"]
    #[inline(always)]
    pub fn pecfg12(&self) -> PECFG12_R {
        PECFG12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PECFG13"]
    #[inline(always)]
    pub fn pecfg13(&self) -> PECFG13_R {
        PECFG13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PECFG14"]
    #[inline(always)]
    pub fn pecfg14(&self) -> PECFG14_R {
        PECFG14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PECFG15"]
    #[inline(always)]
    pub fn pecfg15(&self) -> PECFG15_R {
        PECFG15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PECFG8"]
    #[inline(always)]
    pub fn pecfg8(&mut self) -> PECFG8_W {
        PECFG8_W { w: self }
    }
    #[doc = "Bits 4:7 - PECFG9"]
    #[inline(always)]
    pub fn pecfg9(&mut self) -> PECFG9_W {
        PECFG9_W { w: self }
    }
    #[doc = "Bits 8:11 - PECFG10"]
    #[inline(always)]
    pub fn pecfg10(&mut self) -> PECFG10_W {
        PECFG10_W { w: self }
    }
    #[doc = "Bits 12:15 - PECFG11"]
    #[inline(always)]
    pub fn pecfg11(&mut self) -> PECFG11_W {
        PECFG11_W { w: self }
    }
    #[doc = "Bits 16:19 - PECFG12"]
    #[inline(always)]
    pub fn pecfg12(&mut self) -> PECFG12_W {
        PECFG12_W { w: self }
    }
    #[doc = "Bits 20:23 - PECFG13"]
    #[inline(always)]
    pub fn pecfg13(&mut self) -> PECFG13_W {
        PECFG13_W { w: self }
    }
    #[doc = "Bits 24:27 - PECFG14"]
    #[inline(always)]
    pub fn pecfg14(&mut self) -> PECFG14_W {
        PECFG14_W { w: self }
    }
    #[doc = "Bits 28:31 - PECFG15"]
    #[inline(always)]
    pub fn pecfg15(&mut self) -> PECFG15_W {
        PECFG15_W { w: self }
    }
}

#[doc = "Reader of register AFIO_GPBCFGHR"]
pub type R = crate::R<u32, super::AFIO_GPBCFGHR>;
#[doc = "Writer for register AFIO_GPBCFGHR"]
pub type W = crate::W<u32, super::AFIO_GPBCFGHR>;
#[doc = "Register AFIO_GPBCFGHR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFIO_GPBCFGHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PBCFG8`"]
pub type PBCFG8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBCFG8`"]
pub struct PBCFG8_W<'a> {
    w: &'a mut W,
}
impl<'a> PBCFG8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PBCFG9`"]
pub type PBCFG9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBCFG9`"]
pub struct PBCFG9_W<'a> {
    w: &'a mut W,
}
impl<'a> PBCFG9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PBCFG10`"]
pub type PBCFG10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBCFG10`"]
pub struct PBCFG10_W<'a> {
    w: &'a mut W,
}
impl<'a> PBCFG10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PBCFG11`"]
pub type PBCFG11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBCFG11`"]
pub struct PBCFG11_W<'a> {
    w: &'a mut W,
}
impl<'a> PBCFG11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `PBCFG12`"]
pub type PBCFG12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBCFG12`"]
pub struct PBCFG12_W<'a> {
    w: &'a mut W,
}
impl<'a> PBCFG12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PBCFG13`"]
pub type PBCFG13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBCFG13`"]
pub struct PBCFG13_W<'a> {
    w: &'a mut W,
}
impl<'a> PBCFG13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PBCFG14`"]
pub type PBCFG14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBCFG14`"]
pub struct PBCFG14_W<'a> {
    w: &'a mut W,
}
impl<'a> PBCFG14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `PBCFG15`"]
pub type PBCFG15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBCFG15`"]
pub struct PBCFG15_W<'a> {
    w: &'a mut W,
}
impl<'a> PBCFG15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PBCFG8"]
    #[inline(always)]
    pub fn pbcfg8(&self) -> PBCFG8_R {
        PBCFG8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PBCFG9"]
    #[inline(always)]
    pub fn pbcfg9(&self) -> PBCFG9_R {
        PBCFG9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PBCFG10"]
    #[inline(always)]
    pub fn pbcfg10(&self) -> PBCFG10_R {
        PBCFG10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PBCFG11"]
    #[inline(always)]
    pub fn pbcfg11(&self) -> PBCFG11_R {
        PBCFG11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PBCFG12"]
    #[inline(always)]
    pub fn pbcfg12(&self) -> PBCFG12_R {
        PBCFG12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PBCFG13"]
    #[inline(always)]
    pub fn pbcfg13(&self) -> PBCFG13_R {
        PBCFG13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PBCFG14"]
    #[inline(always)]
    pub fn pbcfg14(&self) -> PBCFG14_R {
        PBCFG14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PBCFG15"]
    #[inline(always)]
    pub fn pbcfg15(&self) -> PBCFG15_R {
        PBCFG15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PBCFG8"]
    #[inline(always)]
    pub fn pbcfg8(&mut self) -> PBCFG8_W {
        PBCFG8_W { w: self }
    }
    #[doc = "Bits 4:7 - PBCFG9"]
    #[inline(always)]
    pub fn pbcfg9(&mut self) -> PBCFG9_W {
        PBCFG9_W { w: self }
    }
    #[doc = "Bits 8:11 - PBCFG10"]
    #[inline(always)]
    pub fn pbcfg10(&mut self) -> PBCFG10_W {
        PBCFG10_W { w: self }
    }
    #[doc = "Bits 12:15 - PBCFG11"]
    #[inline(always)]
    pub fn pbcfg11(&mut self) -> PBCFG11_W {
        PBCFG11_W { w: self }
    }
    #[doc = "Bits 16:19 - PBCFG12"]
    #[inline(always)]
    pub fn pbcfg12(&mut self) -> PBCFG12_W {
        PBCFG12_W { w: self }
    }
    #[doc = "Bits 20:23 - PBCFG13"]
    #[inline(always)]
    pub fn pbcfg13(&mut self) -> PBCFG13_W {
        PBCFG13_W { w: self }
    }
    #[doc = "Bits 24:27 - PBCFG14"]
    #[inline(always)]
    pub fn pbcfg14(&mut self) -> PBCFG14_W {
        PBCFG14_W { w: self }
    }
    #[doc = "Bits 28:31 - PBCFG15"]
    #[inline(always)]
    pub fn pbcfg15(&mut self) -> PBCFG15_W {
        PBCFG15_W { w: self }
    }
}

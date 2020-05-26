#[doc = "Reader of register AFIO_GPCCFGHR"]
pub type R = crate::R<u32, super::AFIO_GPCCFGHR>;
#[doc = "Writer for register AFIO_GPCCFGHR"]
pub type W = crate::W<u32, super::AFIO_GPCCFGHR>;
#[doc = "Register AFIO_GPCCFGHR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFIO_GPCCFGHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCCFG8`"]
pub type PCCFG8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCCFG8`"]
pub struct PCCFG8_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCFG8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PCCFG9`"]
pub type PCCFG9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCCFG9`"]
pub struct PCCFG9_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCFG9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PCCFG10`"]
pub type PCCFG10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCCFG10`"]
pub struct PCCFG10_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCFG10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PCCFG11`"]
pub type PCCFG11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCCFG11`"]
pub struct PCCFG11_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCFG11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `PCCFG12`"]
pub type PCCFG12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCCFG12`"]
pub struct PCCFG12_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCFG12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PCCFG13`"]
pub type PCCFG13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCCFG13`"]
pub struct PCCFG13_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCFG13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PCCFG14`"]
pub type PCCFG14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCCFG14`"]
pub struct PCCFG14_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCFG14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `PCCFG15`"]
pub type PCCFG15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCCFG15`"]
pub struct PCCFG15_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCFG15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PCCFG8"]
    #[inline(always)]
    pub fn pccfg8(&self) -> PCCFG8_R {
        PCCFG8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PCCFG9"]
    #[inline(always)]
    pub fn pccfg9(&self) -> PCCFG9_R {
        PCCFG9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PCCFG10"]
    #[inline(always)]
    pub fn pccfg10(&self) -> PCCFG10_R {
        PCCFG10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PCCFG11"]
    #[inline(always)]
    pub fn pccfg11(&self) -> PCCFG11_R {
        PCCFG11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PCCFG12"]
    #[inline(always)]
    pub fn pccfg12(&self) -> PCCFG12_R {
        PCCFG12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PCCFG13"]
    #[inline(always)]
    pub fn pccfg13(&self) -> PCCFG13_R {
        PCCFG13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PCCFG14"]
    #[inline(always)]
    pub fn pccfg14(&self) -> PCCFG14_R {
        PCCFG14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PCCFG15"]
    #[inline(always)]
    pub fn pccfg15(&self) -> PCCFG15_R {
        PCCFG15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PCCFG8"]
    #[inline(always)]
    pub fn pccfg8(&mut self) -> PCCFG8_W {
        PCCFG8_W { w: self }
    }
    #[doc = "Bits 4:7 - PCCFG9"]
    #[inline(always)]
    pub fn pccfg9(&mut self) -> PCCFG9_W {
        PCCFG9_W { w: self }
    }
    #[doc = "Bits 8:11 - PCCFG10"]
    #[inline(always)]
    pub fn pccfg10(&mut self) -> PCCFG10_W {
        PCCFG10_W { w: self }
    }
    #[doc = "Bits 12:15 - PCCFG11"]
    #[inline(always)]
    pub fn pccfg11(&mut self) -> PCCFG11_W {
        PCCFG11_W { w: self }
    }
    #[doc = "Bits 16:19 - PCCFG12"]
    #[inline(always)]
    pub fn pccfg12(&mut self) -> PCCFG12_W {
        PCCFG12_W { w: self }
    }
    #[doc = "Bits 20:23 - PCCFG13"]
    #[inline(always)]
    pub fn pccfg13(&mut self) -> PCCFG13_W {
        PCCFG13_W { w: self }
    }
    #[doc = "Bits 24:27 - PCCFG14"]
    #[inline(always)]
    pub fn pccfg14(&mut self) -> PCCFG14_W {
        PCCFG14_W { w: self }
    }
    #[doc = "Bits 28:31 - PCCFG15"]
    #[inline(always)]
    pub fn pccfg15(&mut self) -> PCCFG15_W {
        PCCFG15_W { w: self }
    }
}

#[doc = "Reader of register AFIO_GPBCFGLR"]
pub type R = crate::R<u32, super::AFIO_GPBCFGLR>;
#[doc = "Writer for register AFIO_GPBCFGLR"]
pub type W = crate::W<u32, super::AFIO_GPBCFGLR>;
#[doc = "Register AFIO_GPBCFGLR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFIO_GPBCFGLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PBCFG0`"]
pub type PBCFG0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBCFG0`"]
pub struct PBCFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> PBCFG0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PBCFG1`"]
pub type PBCFG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBCFG1`"]
pub struct PBCFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> PBCFG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PBCFG2`"]
pub type PBCFG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBCFG2`"]
pub struct PBCFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> PBCFG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PBCFG3`"]
pub type PBCFG3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBCFG3`"]
pub struct PBCFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> PBCFG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `PBCFG4`"]
pub type PBCFG4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBCFG4`"]
pub struct PBCFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> PBCFG4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PBCFG5`"]
pub type PBCFG5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBCFG5`"]
pub struct PBCFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> PBCFG5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PBCFG6`"]
pub type PBCFG6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBCFG6`"]
pub struct PBCFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> PBCFG6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `PBCFG7`"]
pub type PBCFG7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PBCFG7`"]
pub struct PBCFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> PBCFG7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PBCFG0"]
    #[inline(always)]
    pub fn pbcfg0(&self) -> PBCFG0_R {
        PBCFG0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PBCFG1"]
    #[inline(always)]
    pub fn pbcfg1(&self) -> PBCFG1_R {
        PBCFG1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PBCFG2"]
    #[inline(always)]
    pub fn pbcfg2(&self) -> PBCFG2_R {
        PBCFG2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PBCFG3"]
    #[inline(always)]
    pub fn pbcfg3(&self) -> PBCFG3_R {
        PBCFG3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PBCFG4"]
    #[inline(always)]
    pub fn pbcfg4(&self) -> PBCFG4_R {
        PBCFG4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PBCFG5"]
    #[inline(always)]
    pub fn pbcfg5(&self) -> PBCFG5_R {
        PBCFG5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PBCFG6"]
    #[inline(always)]
    pub fn pbcfg6(&self) -> PBCFG6_R {
        PBCFG6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PBCFG7"]
    #[inline(always)]
    pub fn pbcfg7(&self) -> PBCFG7_R {
        PBCFG7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PBCFG0"]
    #[inline(always)]
    pub fn pbcfg0(&mut self) -> PBCFG0_W {
        PBCFG0_W { w: self }
    }
    #[doc = "Bits 4:7 - PBCFG1"]
    #[inline(always)]
    pub fn pbcfg1(&mut self) -> PBCFG1_W {
        PBCFG1_W { w: self }
    }
    #[doc = "Bits 8:11 - PBCFG2"]
    #[inline(always)]
    pub fn pbcfg2(&mut self) -> PBCFG2_W {
        PBCFG2_W { w: self }
    }
    #[doc = "Bits 12:15 - PBCFG3"]
    #[inline(always)]
    pub fn pbcfg3(&mut self) -> PBCFG3_W {
        PBCFG3_W { w: self }
    }
    #[doc = "Bits 16:19 - PBCFG4"]
    #[inline(always)]
    pub fn pbcfg4(&mut self) -> PBCFG4_W {
        PBCFG4_W { w: self }
    }
    #[doc = "Bits 20:23 - PBCFG5"]
    #[inline(always)]
    pub fn pbcfg5(&mut self) -> PBCFG5_W {
        PBCFG5_W { w: self }
    }
    #[doc = "Bits 24:27 - PBCFG6"]
    #[inline(always)]
    pub fn pbcfg6(&mut self) -> PBCFG6_W {
        PBCFG6_W { w: self }
    }
    #[doc = "Bits 28:31 - PBCFG7"]
    #[inline(always)]
    pub fn pbcfg7(&mut self) -> PBCFG7_W {
        PBCFG7_W { w: self }
    }
}

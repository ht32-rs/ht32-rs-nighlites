#[doc = "Reader of register AFIO_GPACFGLR"]
pub type R = crate::R<u32, super::AFIO_GPACFGLR>;
#[doc = "Writer for register AFIO_GPACFGLR"]
pub type W = crate::W<u32, super::AFIO_GPACFGLR>;
#[doc = "Register AFIO_GPACFGLR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFIO_GPACFGLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PACFG0`"]
pub type PACFG0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACFG0`"]
pub struct PACFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> PACFG0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PACFG1`"]
pub type PACFG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACFG1`"]
pub struct PACFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> PACFG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PACFG2`"]
pub type PACFG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACFG2`"]
pub struct PACFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> PACFG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PACFG3`"]
pub type PACFG3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACFG3`"]
pub struct PACFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> PACFG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `PACFG4`"]
pub type PACFG4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACFG4`"]
pub struct PACFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> PACFG4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PACFG5`"]
pub type PACFG5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACFG5`"]
pub struct PACFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> PACFG5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PACFG6`"]
pub type PACFG6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACFG6`"]
pub struct PACFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> PACFG6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `PACFG7`"]
pub type PACFG7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PACFG7`"]
pub struct PACFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> PACFG7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PACFG0"]
    #[inline(always)]
    pub fn pacfg0(&self) -> PACFG0_R {
        PACFG0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PACFG1"]
    #[inline(always)]
    pub fn pacfg1(&self) -> PACFG1_R {
        PACFG1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PACFG2"]
    #[inline(always)]
    pub fn pacfg2(&self) -> PACFG2_R {
        PACFG2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PACFG3"]
    #[inline(always)]
    pub fn pacfg3(&self) -> PACFG3_R {
        PACFG3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PACFG4"]
    #[inline(always)]
    pub fn pacfg4(&self) -> PACFG4_R {
        PACFG4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PACFG5"]
    #[inline(always)]
    pub fn pacfg5(&self) -> PACFG5_R {
        PACFG5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PACFG6"]
    #[inline(always)]
    pub fn pacfg6(&self) -> PACFG6_R {
        PACFG6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PACFG7"]
    #[inline(always)]
    pub fn pacfg7(&self) -> PACFG7_R {
        PACFG7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PACFG0"]
    #[inline(always)]
    pub fn pacfg0(&mut self) -> PACFG0_W {
        PACFG0_W { w: self }
    }
    #[doc = "Bits 4:7 - PACFG1"]
    #[inline(always)]
    pub fn pacfg1(&mut self) -> PACFG1_W {
        PACFG1_W { w: self }
    }
    #[doc = "Bits 8:11 - PACFG2"]
    #[inline(always)]
    pub fn pacfg2(&mut self) -> PACFG2_W {
        PACFG2_W { w: self }
    }
    #[doc = "Bits 12:15 - PACFG3"]
    #[inline(always)]
    pub fn pacfg3(&mut self) -> PACFG3_W {
        PACFG3_W { w: self }
    }
    #[doc = "Bits 16:19 - PACFG4"]
    #[inline(always)]
    pub fn pacfg4(&mut self) -> PACFG4_W {
        PACFG4_W { w: self }
    }
    #[doc = "Bits 20:23 - PACFG5"]
    #[inline(always)]
    pub fn pacfg5(&mut self) -> PACFG5_W {
        PACFG5_W { w: self }
    }
    #[doc = "Bits 24:27 - PACFG6"]
    #[inline(always)]
    pub fn pacfg6(&mut self) -> PACFG6_W {
        PACFG6_W { w: self }
    }
    #[doc = "Bits 28:31 - PACFG7"]
    #[inline(always)]
    pub fn pacfg7(&mut self) -> PACFG7_W {
        PACFG7_W { w: self }
    }
}

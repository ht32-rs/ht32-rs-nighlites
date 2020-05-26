#[doc = "Reader of register AFIO_GPDCFGLR"]
pub type R = crate::R<u32, super::AFIO_GPDCFGLR>;
#[doc = "Writer for register AFIO_GPDCFGLR"]
pub type W = crate::W<u32, super::AFIO_GPDCFGLR>;
#[doc = "Register AFIO_GPDCFGLR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFIO_GPDCFGLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDCFG0`"]
pub type PDCFG0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDCFG0`"]
pub struct PDCFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PDCFG1`"]
pub type PDCFG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDCFG1`"]
pub struct PDCFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PDCFG2`"]
pub type PDCFG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDCFG2`"]
pub struct PDCFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PDCFG3`"]
pub type PDCFG3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDCFG3`"]
pub struct PDCFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `PDCFG4`"]
pub type PDCFG4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDCFG4`"]
pub struct PDCFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PDCFG5`"]
pub type PDCFG5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDCFG5`"]
pub struct PDCFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PDCFG6`"]
pub type PDCFG6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDCFG6`"]
pub struct PDCFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `PDCFG7`"]
pub type PDCFG7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDCFG7`"]
pub struct PDCFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> PDCFG7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PDCFG0"]
    #[inline(always)]
    pub fn pdcfg0(&self) -> PDCFG0_R {
        PDCFG0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PDCFG1"]
    #[inline(always)]
    pub fn pdcfg1(&self) -> PDCFG1_R {
        PDCFG1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PDCFG2"]
    #[inline(always)]
    pub fn pdcfg2(&self) -> PDCFG2_R {
        PDCFG2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PDCFG3"]
    #[inline(always)]
    pub fn pdcfg3(&self) -> PDCFG3_R {
        PDCFG3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PDCFG4"]
    #[inline(always)]
    pub fn pdcfg4(&self) -> PDCFG4_R {
        PDCFG4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PDCFG5"]
    #[inline(always)]
    pub fn pdcfg5(&self) -> PDCFG5_R {
        PDCFG5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PDCFG6"]
    #[inline(always)]
    pub fn pdcfg6(&self) -> PDCFG6_R {
        PDCFG6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PDCFG7"]
    #[inline(always)]
    pub fn pdcfg7(&self) -> PDCFG7_R {
        PDCFG7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDCFG0"]
    #[inline(always)]
    pub fn pdcfg0(&mut self) -> PDCFG0_W {
        PDCFG0_W { w: self }
    }
    #[doc = "Bits 4:7 - PDCFG1"]
    #[inline(always)]
    pub fn pdcfg1(&mut self) -> PDCFG1_W {
        PDCFG1_W { w: self }
    }
    #[doc = "Bits 8:11 - PDCFG2"]
    #[inline(always)]
    pub fn pdcfg2(&mut self) -> PDCFG2_W {
        PDCFG2_W { w: self }
    }
    #[doc = "Bits 12:15 - PDCFG3"]
    #[inline(always)]
    pub fn pdcfg3(&mut self) -> PDCFG3_W {
        PDCFG3_W { w: self }
    }
    #[doc = "Bits 16:19 - PDCFG4"]
    #[inline(always)]
    pub fn pdcfg4(&mut self) -> PDCFG4_W {
        PDCFG4_W { w: self }
    }
    #[doc = "Bits 20:23 - PDCFG5"]
    #[inline(always)]
    pub fn pdcfg5(&mut self) -> PDCFG5_W {
        PDCFG5_W { w: self }
    }
    #[doc = "Bits 24:27 - PDCFG6"]
    #[inline(always)]
    pub fn pdcfg6(&mut self) -> PDCFG6_W {
        PDCFG6_W { w: self }
    }
    #[doc = "Bits 28:31 - PDCFG7"]
    #[inline(always)]
    pub fn pdcfg7(&mut self) -> PDCFG7_W {
        PDCFG7_W { w: self }
    }
}

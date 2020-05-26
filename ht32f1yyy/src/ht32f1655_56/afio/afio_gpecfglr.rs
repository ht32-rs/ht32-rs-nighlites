#[doc = "Reader of register AFIO_GPECFGLR"]
pub type R = crate::R<u32, super::AFIO_GPECFGLR>;
#[doc = "Writer for register AFIO_GPECFGLR"]
pub type W = crate::W<u32, super::AFIO_GPECFGLR>;
#[doc = "Register AFIO_GPECFGLR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFIO_GPECFGLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PECFG0`"]
pub type PECFG0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PECFG0`"]
pub struct PECFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFG0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PECFG1`"]
pub type PECFG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PECFG1`"]
pub struct PECFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PECFG2`"]
pub type PECFG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PECFG2`"]
pub struct PECFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PECFG3`"]
pub type PECFG3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PECFG3`"]
pub struct PECFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `PECFG4`"]
pub type PECFG4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PECFG4`"]
pub struct PECFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFG4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PECFG5`"]
pub type PECFG5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PECFG5`"]
pub struct PECFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFG5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PECFG6`"]
pub type PECFG6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PECFG6`"]
pub struct PECFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFG6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `PECFG7`"]
pub type PECFG7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PECFG7`"]
pub struct PECFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> PECFG7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PECFG0"]
    #[inline(always)]
    pub fn pecfg0(&self) -> PECFG0_R {
        PECFG0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PECFG1"]
    #[inline(always)]
    pub fn pecfg1(&self) -> PECFG1_R {
        PECFG1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PECFG2"]
    #[inline(always)]
    pub fn pecfg2(&self) -> PECFG2_R {
        PECFG2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PECFG3"]
    #[inline(always)]
    pub fn pecfg3(&self) -> PECFG3_R {
        PECFG3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PECFG4"]
    #[inline(always)]
    pub fn pecfg4(&self) -> PECFG4_R {
        PECFG4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PECFG5"]
    #[inline(always)]
    pub fn pecfg5(&self) -> PECFG5_R {
        PECFG5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PECFG6"]
    #[inline(always)]
    pub fn pecfg6(&self) -> PECFG6_R {
        PECFG6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PECFG7"]
    #[inline(always)]
    pub fn pecfg7(&self) -> PECFG7_R {
        PECFG7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PECFG0"]
    #[inline(always)]
    pub fn pecfg0(&mut self) -> PECFG0_W {
        PECFG0_W { w: self }
    }
    #[doc = "Bits 4:7 - PECFG1"]
    #[inline(always)]
    pub fn pecfg1(&mut self) -> PECFG1_W {
        PECFG1_W { w: self }
    }
    #[doc = "Bits 8:11 - PECFG2"]
    #[inline(always)]
    pub fn pecfg2(&mut self) -> PECFG2_W {
        PECFG2_W { w: self }
    }
    #[doc = "Bits 12:15 - PECFG3"]
    #[inline(always)]
    pub fn pecfg3(&mut self) -> PECFG3_W {
        PECFG3_W { w: self }
    }
    #[doc = "Bits 16:19 - PECFG4"]
    #[inline(always)]
    pub fn pecfg4(&mut self) -> PECFG4_W {
        PECFG4_W { w: self }
    }
    #[doc = "Bits 20:23 - PECFG5"]
    #[inline(always)]
    pub fn pecfg5(&mut self) -> PECFG5_W {
        PECFG5_W { w: self }
    }
    #[doc = "Bits 24:27 - PECFG6"]
    #[inline(always)]
    pub fn pecfg6(&mut self) -> PECFG6_W {
        PECFG6_W { w: self }
    }
    #[doc = "Bits 28:31 - PECFG7"]
    #[inline(always)]
    pub fn pecfg7(&mut self) -> PECFG7_W {
        PECFG7_W { w: self }
    }
}

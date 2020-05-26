#[doc = "Reader of register AFIO_GPCCFGLR"]
pub type R = crate::R<u32, super::AFIO_GPCCFGLR>;
#[doc = "Writer for register AFIO_GPCCFGLR"]
pub type W = crate::W<u32, super::AFIO_GPCCFGLR>;
#[doc = "Register AFIO_GPCCFGLR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFIO_GPCCFGLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCCFG0`"]
pub type PCCFG0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCCFG0`"]
pub struct PCCFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCFG0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PCCFG1`"]
pub type PCCFG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCCFG1`"]
pub struct PCCFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCFG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PCCFG2`"]
pub type PCCFG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCCFG2`"]
pub struct PCCFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCFG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PCCFG3`"]
pub type PCCFG3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCCFG3`"]
pub struct PCCFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCFG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `PCCFG4`"]
pub type PCCFG4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCCFG4`"]
pub struct PCCFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCFG4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PCCFG5`"]
pub type PCCFG5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCCFG5`"]
pub struct PCCFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCFG5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PCCFG6`"]
pub type PCCFG6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCCFG6`"]
pub struct PCCFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCFG6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `PCCFG7`"]
pub type PCCFG7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCCFG7`"]
pub struct PCCFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> PCCFG7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PCCFG0"]
    #[inline(always)]
    pub fn pccfg0(&self) -> PCCFG0_R {
        PCCFG0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PCCFG1"]
    #[inline(always)]
    pub fn pccfg1(&self) -> PCCFG1_R {
        PCCFG1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PCCFG2"]
    #[inline(always)]
    pub fn pccfg2(&self) -> PCCFG2_R {
        PCCFG2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PCCFG3"]
    #[inline(always)]
    pub fn pccfg3(&self) -> PCCFG3_R {
        PCCFG3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PCCFG4"]
    #[inline(always)]
    pub fn pccfg4(&self) -> PCCFG4_R {
        PCCFG4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - PCCFG5"]
    #[inline(always)]
    pub fn pccfg5(&self) -> PCCFG5_R {
        PCCFG5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - PCCFG6"]
    #[inline(always)]
    pub fn pccfg6(&self) -> PCCFG6_R {
        PCCFG6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PCCFG7"]
    #[inline(always)]
    pub fn pccfg7(&self) -> PCCFG7_R {
        PCCFG7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PCCFG0"]
    #[inline(always)]
    pub fn pccfg0(&mut self) -> PCCFG0_W {
        PCCFG0_W { w: self }
    }
    #[doc = "Bits 4:7 - PCCFG1"]
    #[inline(always)]
    pub fn pccfg1(&mut self) -> PCCFG1_W {
        PCCFG1_W { w: self }
    }
    #[doc = "Bits 8:11 - PCCFG2"]
    #[inline(always)]
    pub fn pccfg2(&mut self) -> PCCFG2_W {
        PCCFG2_W { w: self }
    }
    #[doc = "Bits 12:15 - PCCFG3"]
    #[inline(always)]
    pub fn pccfg3(&mut self) -> PCCFG3_W {
        PCCFG3_W { w: self }
    }
    #[doc = "Bits 16:19 - PCCFG4"]
    #[inline(always)]
    pub fn pccfg4(&mut self) -> PCCFG4_W {
        PCCFG4_W { w: self }
    }
    #[doc = "Bits 20:23 - PCCFG5"]
    #[inline(always)]
    pub fn pccfg5(&mut self) -> PCCFG5_W {
        PCCFG5_W { w: self }
    }
    #[doc = "Bits 24:27 - PCCFG6"]
    #[inline(always)]
    pub fn pccfg6(&mut self) -> PCCFG6_W {
        PCCFG6_W { w: self }
    }
    #[doc = "Bits 28:31 - PCCFG7"]
    #[inline(always)]
    pub fn pccfg7(&mut self) -> PCCFG7_W {
        PCCFG7_W { w: self }
    }
}

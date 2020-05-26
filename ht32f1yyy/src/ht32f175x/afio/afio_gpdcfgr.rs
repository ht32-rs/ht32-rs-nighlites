#[doc = "Reader of register AFIO_GPDCFGR"]
pub type R = crate::R<u32, super::AFIO_GPDCFGR>;
#[doc = "Writer for register AFIO_GPDCFGR"]
pub type W = crate::W<u32, super::AFIO_GPDCFGR>;
#[doc = "Register AFIO_GPDCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFIO_GPDCFGR {
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
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
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
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
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
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
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
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
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
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
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
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
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
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
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
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
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
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
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PDCFG0"]
    #[inline(always)]
    pub fn pdcfg0(&self) -> PDCFG0_R {
        PDCFG0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - PDCFG1"]
    #[inline(always)]
    pub fn pdcfg1(&self) -> PDCFG1_R {
        PDCFG1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - PDCFG2"]
    #[inline(always)]
    pub fn pdcfg2(&self) -> PDCFG2_R {
        PDCFG2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - PDCFG3"]
    #[inline(always)]
    pub fn pdcfg3(&self) -> PDCFG3_R {
        PDCFG3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - PDCFG4"]
    #[inline(always)]
    pub fn pdcfg4(&self) -> PDCFG4_R {
        PDCFG4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - PDCFG5"]
    #[inline(always)]
    pub fn pdcfg5(&self) -> PDCFG5_R {
        PDCFG5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - PDCFG6"]
    #[inline(always)]
    pub fn pdcfg6(&self) -> PDCFG6_R {
        PDCFG6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - PDCFG7"]
    #[inline(always)]
    pub fn pdcfg7(&self) -> PDCFG7_R {
        PDCFG7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - PDCFG8"]
    #[inline(always)]
    pub fn pdcfg8(&self) -> PDCFG8_R {
        PDCFG8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - PDCFG9"]
    #[inline(always)]
    pub fn pdcfg9(&self) -> PDCFG9_R {
        PDCFG9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - PDCFG10"]
    #[inline(always)]
    pub fn pdcfg10(&self) -> PDCFG10_R {
        PDCFG10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - PDCFG11"]
    #[inline(always)]
    pub fn pdcfg11(&self) -> PDCFG11_R {
        PDCFG11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - PDCFG12"]
    #[inline(always)]
    pub fn pdcfg12(&self) -> PDCFG12_R {
        PDCFG12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - PDCFG13"]
    #[inline(always)]
    pub fn pdcfg13(&self) -> PDCFG13_R {
        PDCFG13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - PDCFG14"]
    #[inline(always)]
    pub fn pdcfg14(&self) -> PDCFG14_R {
        PDCFG14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - PDCFG15"]
    #[inline(always)]
    pub fn pdcfg15(&self) -> PDCFG15_R {
        PDCFG15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PDCFG0"]
    #[inline(always)]
    pub fn pdcfg0(&mut self) -> PDCFG0_W {
        PDCFG0_W { w: self }
    }
    #[doc = "Bits 2:3 - PDCFG1"]
    #[inline(always)]
    pub fn pdcfg1(&mut self) -> PDCFG1_W {
        PDCFG1_W { w: self }
    }
    #[doc = "Bits 4:5 - PDCFG2"]
    #[inline(always)]
    pub fn pdcfg2(&mut self) -> PDCFG2_W {
        PDCFG2_W { w: self }
    }
    #[doc = "Bits 6:7 - PDCFG3"]
    #[inline(always)]
    pub fn pdcfg3(&mut self) -> PDCFG3_W {
        PDCFG3_W { w: self }
    }
    #[doc = "Bits 8:9 - PDCFG4"]
    #[inline(always)]
    pub fn pdcfg4(&mut self) -> PDCFG4_W {
        PDCFG4_W { w: self }
    }
    #[doc = "Bits 10:11 - PDCFG5"]
    #[inline(always)]
    pub fn pdcfg5(&mut self) -> PDCFG5_W {
        PDCFG5_W { w: self }
    }
    #[doc = "Bits 12:13 - PDCFG6"]
    #[inline(always)]
    pub fn pdcfg6(&mut self) -> PDCFG6_W {
        PDCFG6_W { w: self }
    }
    #[doc = "Bits 14:15 - PDCFG7"]
    #[inline(always)]
    pub fn pdcfg7(&mut self) -> PDCFG7_W {
        PDCFG7_W { w: self }
    }
    #[doc = "Bits 16:17 - PDCFG8"]
    #[inline(always)]
    pub fn pdcfg8(&mut self) -> PDCFG8_W {
        PDCFG8_W { w: self }
    }
    #[doc = "Bits 18:19 - PDCFG9"]
    #[inline(always)]
    pub fn pdcfg9(&mut self) -> PDCFG9_W {
        PDCFG9_W { w: self }
    }
    #[doc = "Bits 20:21 - PDCFG10"]
    #[inline(always)]
    pub fn pdcfg10(&mut self) -> PDCFG10_W {
        PDCFG10_W { w: self }
    }
    #[doc = "Bits 22:23 - PDCFG11"]
    #[inline(always)]
    pub fn pdcfg11(&mut self) -> PDCFG11_W {
        PDCFG11_W { w: self }
    }
    #[doc = "Bits 24:25 - PDCFG12"]
    #[inline(always)]
    pub fn pdcfg12(&mut self) -> PDCFG12_W {
        PDCFG12_W { w: self }
    }
    #[doc = "Bits 26:27 - PDCFG13"]
    #[inline(always)]
    pub fn pdcfg13(&mut self) -> PDCFG13_W {
        PDCFG13_W { w: self }
    }
    #[doc = "Bits 28:29 - PDCFG14"]
    #[inline(always)]
    pub fn pdcfg14(&mut self) -> PDCFG14_W {
        PDCFG14_W { w: self }
    }
    #[doc = "Bits 30:31 - PDCFG15"]
    #[inline(always)]
    pub fn pdcfg15(&mut self) -> PDCFG15_W {
        PDCFG15_W { w: self }
    }
}

#[doc = "Reader of register AFIO_GPACFGR"]
pub type R = crate::R<u32, super::AFIO_GPACFGR>;
#[doc = "Writer for register AFIO_GPACFGR"]
pub type W = crate::W<u32, super::AFIO_GPACFGR>;
#[doc = "Register AFIO_GPACFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFIO_GPACFGR {
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
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
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
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
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
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
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
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
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
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
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
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
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
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
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
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
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
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
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PACFG0"]
    #[inline(always)]
    pub fn pacfg0(&self) -> PACFG0_R {
        PACFG0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - PACFG1"]
    #[inline(always)]
    pub fn pacfg1(&self) -> PACFG1_R {
        PACFG1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - PACFG2"]
    #[inline(always)]
    pub fn pacfg2(&self) -> PACFG2_R {
        PACFG2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - PACFG3"]
    #[inline(always)]
    pub fn pacfg3(&self) -> PACFG3_R {
        PACFG3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - PACFG4"]
    #[inline(always)]
    pub fn pacfg4(&self) -> PACFG4_R {
        PACFG4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - PACFG5"]
    #[inline(always)]
    pub fn pacfg5(&self) -> PACFG5_R {
        PACFG5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - PACFG6"]
    #[inline(always)]
    pub fn pacfg6(&self) -> PACFG6_R {
        PACFG6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - PACFG7"]
    #[inline(always)]
    pub fn pacfg7(&self) -> PACFG7_R {
        PACFG7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - PACFG8"]
    #[inline(always)]
    pub fn pacfg8(&self) -> PACFG8_R {
        PACFG8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - PACFG9"]
    #[inline(always)]
    pub fn pacfg9(&self) -> PACFG9_R {
        PACFG9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - PACFG10"]
    #[inline(always)]
    pub fn pacfg10(&self) -> PACFG10_R {
        PACFG10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - PACFG11"]
    #[inline(always)]
    pub fn pacfg11(&self) -> PACFG11_R {
        PACFG11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - PACFG12"]
    #[inline(always)]
    pub fn pacfg12(&self) -> PACFG12_R {
        PACFG12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - PACFG13"]
    #[inline(always)]
    pub fn pacfg13(&self) -> PACFG13_R {
        PACFG13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - PACFG14"]
    #[inline(always)]
    pub fn pacfg14(&self) -> PACFG14_R {
        PACFG14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - PACFG15"]
    #[inline(always)]
    pub fn pacfg15(&self) -> PACFG15_R {
        PACFG15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PACFG0"]
    #[inline(always)]
    pub fn pacfg0(&mut self) -> PACFG0_W {
        PACFG0_W { w: self }
    }
    #[doc = "Bits 2:3 - PACFG1"]
    #[inline(always)]
    pub fn pacfg1(&mut self) -> PACFG1_W {
        PACFG1_W { w: self }
    }
    #[doc = "Bits 4:5 - PACFG2"]
    #[inline(always)]
    pub fn pacfg2(&mut self) -> PACFG2_W {
        PACFG2_W { w: self }
    }
    #[doc = "Bits 6:7 - PACFG3"]
    #[inline(always)]
    pub fn pacfg3(&mut self) -> PACFG3_W {
        PACFG3_W { w: self }
    }
    #[doc = "Bits 8:9 - PACFG4"]
    #[inline(always)]
    pub fn pacfg4(&mut self) -> PACFG4_W {
        PACFG4_W { w: self }
    }
    #[doc = "Bits 10:11 - PACFG5"]
    #[inline(always)]
    pub fn pacfg5(&mut self) -> PACFG5_W {
        PACFG5_W { w: self }
    }
    #[doc = "Bits 12:13 - PACFG6"]
    #[inline(always)]
    pub fn pacfg6(&mut self) -> PACFG6_W {
        PACFG6_W { w: self }
    }
    #[doc = "Bits 14:15 - PACFG7"]
    #[inline(always)]
    pub fn pacfg7(&mut self) -> PACFG7_W {
        PACFG7_W { w: self }
    }
    #[doc = "Bits 16:17 - PACFG8"]
    #[inline(always)]
    pub fn pacfg8(&mut self) -> PACFG8_W {
        PACFG8_W { w: self }
    }
    #[doc = "Bits 18:19 - PACFG9"]
    #[inline(always)]
    pub fn pacfg9(&mut self) -> PACFG9_W {
        PACFG9_W { w: self }
    }
    #[doc = "Bits 20:21 - PACFG10"]
    #[inline(always)]
    pub fn pacfg10(&mut self) -> PACFG10_W {
        PACFG10_W { w: self }
    }
    #[doc = "Bits 22:23 - PACFG11"]
    #[inline(always)]
    pub fn pacfg11(&mut self) -> PACFG11_W {
        PACFG11_W { w: self }
    }
    #[doc = "Bits 24:25 - PACFG12"]
    #[inline(always)]
    pub fn pacfg12(&mut self) -> PACFG12_W {
        PACFG12_W { w: self }
    }
    #[doc = "Bits 26:27 - PACFG13"]
    #[inline(always)]
    pub fn pacfg13(&mut self) -> PACFG13_W {
        PACFG13_W { w: self }
    }
    #[doc = "Bits 28:29 - PACFG14"]
    #[inline(always)]
    pub fn pacfg14(&mut self) -> PACFG14_W {
        PACFG14_W { w: self }
    }
    #[doc = "Bits 30:31 - PACFG15"]
    #[inline(always)]
    pub fn pacfg15(&mut self) -> PACFG15_W {
        PACFG15_W { w: self }
    }
}

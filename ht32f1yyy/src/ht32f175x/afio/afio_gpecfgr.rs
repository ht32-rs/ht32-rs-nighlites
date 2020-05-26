#[doc = "Reader of register AFIO_GPECFGR"]
pub type R = crate::R<u32, super::AFIO_GPECFGR>;
#[doc = "Writer for register AFIO_GPECFGR"]
pub type W = crate::W<u32, super::AFIO_GPECFGR>;
#[doc = "Register AFIO_GPECFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFIO_GPECFGR {
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
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
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
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
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
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
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
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
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
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
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
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
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
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
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
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
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
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
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PECFG0"]
    #[inline(always)]
    pub fn pecfg0(&self) -> PECFG0_R {
        PECFG0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - PECFG1"]
    #[inline(always)]
    pub fn pecfg1(&self) -> PECFG1_R {
        PECFG1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - PECFG2"]
    #[inline(always)]
    pub fn pecfg2(&self) -> PECFG2_R {
        PECFG2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - PECFG3"]
    #[inline(always)]
    pub fn pecfg3(&self) -> PECFG3_R {
        PECFG3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - PECFG4"]
    #[inline(always)]
    pub fn pecfg4(&self) -> PECFG4_R {
        PECFG4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - PECFG5"]
    #[inline(always)]
    pub fn pecfg5(&self) -> PECFG5_R {
        PECFG5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - PECFG6"]
    #[inline(always)]
    pub fn pecfg6(&self) -> PECFG6_R {
        PECFG6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - PECFG7"]
    #[inline(always)]
    pub fn pecfg7(&self) -> PECFG7_R {
        PECFG7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - PECFG8"]
    #[inline(always)]
    pub fn pecfg8(&self) -> PECFG8_R {
        PECFG8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - PECFG9"]
    #[inline(always)]
    pub fn pecfg9(&self) -> PECFG9_R {
        PECFG9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - PECFG10"]
    #[inline(always)]
    pub fn pecfg10(&self) -> PECFG10_R {
        PECFG10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - PECFG11"]
    #[inline(always)]
    pub fn pecfg11(&self) -> PECFG11_R {
        PECFG11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - PECFG12"]
    #[inline(always)]
    pub fn pecfg12(&self) -> PECFG12_R {
        PECFG12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - PECFG13"]
    #[inline(always)]
    pub fn pecfg13(&self) -> PECFG13_R {
        PECFG13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - PECFG14"]
    #[inline(always)]
    pub fn pecfg14(&self) -> PECFG14_R {
        PECFG14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - PECFG15"]
    #[inline(always)]
    pub fn pecfg15(&self) -> PECFG15_R {
        PECFG15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PECFG0"]
    #[inline(always)]
    pub fn pecfg0(&mut self) -> PECFG0_W {
        PECFG0_W { w: self }
    }
    #[doc = "Bits 2:3 - PECFG1"]
    #[inline(always)]
    pub fn pecfg1(&mut self) -> PECFG1_W {
        PECFG1_W { w: self }
    }
    #[doc = "Bits 4:5 - PECFG2"]
    #[inline(always)]
    pub fn pecfg2(&mut self) -> PECFG2_W {
        PECFG2_W { w: self }
    }
    #[doc = "Bits 6:7 - PECFG3"]
    #[inline(always)]
    pub fn pecfg3(&mut self) -> PECFG3_W {
        PECFG3_W { w: self }
    }
    #[doc = "Bits 8:9 - PECFG4"]
    #[inline(always)]
    pub fn pecfg4(&mut self) -> PECFG4_W {
        PECFG4_W { w: self }
    }
    #[doc = "Bits 10:11 - PECFG5"]
    #[inline(always)]
    pub fn pecfg5(&mut self) -> PECFG5_W {
        PECFG5_W { w: self }
    }
    #[doc = "Bits 12:13 - PECFG6"]
    #[inline(always)]
    pub fn pecfg6(&mut self) -> PECFG6_W {
        PECFG6_W { w: self }
    }
    #[doc = "Bits 14:15 - PECFG7"]
    #[inline(always)]
    pub fn pecfg7(&mut self) -> PECFG7_W {
        PECFG7_W { w: self }
    }
    #[doc = "Bits 16:17 - PECFG8"]
    #[inline(always)]
    pub fn pecfg8(&mut self) -> PECFG8_W {
        PECFG8_W { w: self }
    }
    #[doc = "Bits 18:19 - PECFG9"]
    #[inline(always)]
    pub fn pecfg9(&mut self) -> PECFG9_W {
        PECFG9_W { w: self }
    }
    #[doc = "Bits 20:21 - PECFG10"]
    #[inline(always)]
    pub fn pecfg10(&mut self) -> PECFG10_W {
        PECFG10_W { w: self }
    }
    #[doc = "Bits 22:23 - PECFG11"]
    #[inline(always)]
    pub fn pecfg11(&mut self) -> PECFG11_W {
        PECFG11_W { w: self }
    }
    #[doc = "Bits 24:25 - PECFG12"]
    #[inline(always)]
    pub fn pecfg12(&mut self) -> PECFG12_W {
        PECFG12_W { w: self }
    }
    #[doc = "Bits 26:27 - PECFG13"]
    #[inline(always)]
    pub fn pecfg13(&mut self) -> PECFG13_W {
        PECFG13_W { w: self }
    }
    #[doc = "Bits 28:29 - PECFG14"]
    #[inline(always)]
    pub fn pecfg14(&mut self) -> PECFG14_W {
        PECFG14_W { w: self }
    }
    #[doc = "Bits 30:31 - PECFG15"]
    #[inline(always)]
    pub fn pecfg15(&mut self) -> PECFG15_W {
        PECFG15_W { w: self }
    }
}

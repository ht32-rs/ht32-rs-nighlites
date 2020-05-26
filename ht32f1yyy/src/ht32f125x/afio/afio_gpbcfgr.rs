#[doc = "Reader of register AFIO_GPBCFGR"]
pub type R = crate::R<u32, super::AFIO_GPBCFGR>;
#[doc = "Writer for register AFIO_GPBCFGR"]
pub type W = crate::W<u32, super::AFIO_GPBCFGR>;
#[doc = "Register AFIO_GPBCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFIO_GPBCFGR {
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
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
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
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
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
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
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
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
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
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
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
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
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
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
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
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
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
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
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PBCFG0"]
    #[inline(always)]
    pub fn pbcfg0(&self) -> PBCFG0_R {
        PBCFG0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - PBCFG1"]
    #[inline(always)]
    pub fn pbcfg1(&self) -> PBCFG1_R {
        PBCFG1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - PBCFG2"]
    #[inline(always)]
    pub fn pbcfg2(&self) -> PBCFG2_R {
        PBCFG2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - PBCFG3"]
    #[inline(always)]
    pub fn pbcfg3(&self) -> PBCFG3_R {
        PBCFG3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - PBCFG4"]
    #[inline(always)]
    pub fn pbcfg4(&self) -> PBCFG4_R {
        PBCFG4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - PBCFG5"]
    #[inline(always)]
    pub fn pbcfg5(&self) -> PBCFG5_R {
        PBCFG5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - PBCFG6"]
    #[inline(always)]
    pub fn pbcfg6(&self) -> PBCFG6_R {
        PBCFG6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - PBCFG7"]
    #[inline(always)]
    pub fn pbcfg7(&self) -> PBCFG7_R {
        PBCFG7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - PBCFG8"]
    #[inline(always)]
    pub fn pbcfg8(&self) -> PBCFG8_R {
        PBCFG8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - PBCFG9"]
    #[inline(always)]
    pub fn pbcfg9(&self) -> PBCFG9_R {
        PBCFG9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - PBCFG10"]
    #[inline(always)]
    pub fn pbcfg10(&self) -> PBCFG10_R {
        PBCFG10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - PBCFG11"]
    #[inline(always)]
    pub fn pbcfg11(&self) -> PBCFG11_R {
        PBCFG11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - PBCFG12"]
    #[inline(always)]
    pub fn pbcfg12(&self) -> PBCFG12_R {
        PBCFG12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - PBCFG13"]
    #[inline(always)]
    pub fn pbcfg13(&self) -> PBCFG13_R {
        PBCFG13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - PBCFG14"]
    #[inline(always)]
    pub fn pbcfg14(&self) -> PBCFG14_R {
        PBCFG14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - PBCFG15"]
    #[inline(always)]
    pub fn pbcfg15(&self) -> PBCFG15_R {
        PBCFG15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PBCFG0"]
    #[inline(always)]
    pub fn pbcfg0(&mut self) -> PBCFG0_W {
        PBCFG0_W { w: self }
    }
    #[doc = "Bits 2:3 - PBCFG1"]
    #[inline(always)]
    pub fn pbcfg1(&mut self) -> PBCFG1_W {
        PBCFG1_W { w: self }
    }
    #[doc = "Bits 4:5 - PBCFG2"]
    #[inline(always)]
    pub fn pbcfg2(&mut self) -> PBCFG2_W {
        PBCFG2_W { w: self }
    }
    #[doc = "Bits 6:7 - PBCFG3"]
    #[inline(always)]
    pub fn pbcfg3(&mut self) -> PBCFG3_W {
        PBCFG3_W { w: self }
    }
    #[doc = "Bits 8:9 - PBCFG4"]
    #[inline(always)]
    pub fn pbcfg4(&mut self) -> PBCFG4_W {
        PBCFG4_W { w: self }
    }
    #[doc = "Bits 10:11 - PBCFG5"]
    #[inline(always)]
    pub fn pbcfg5(&mut self) -> PBCFG5_W {
        PBCFG5_W { w: self }
    }
    #[doc = "Bits 12:13 - PBCFG6"]
    #[inline(always)]
    pub fn pbcfg6(&mut self) -> PBCFG6_W {
        PBCFG6_W { w: self }
    }
    #[doc = "Bits 14:15 - PBCFG7"]
    #[inline(always)]
    pub fn pbcfg7(&mut self) -> PBCFG7_W {
        PBCFG7_W { w: self }
    }
    #[doc = "Bits 16:17 - PBCFG8"]
    #[inline(always)]
    pub fn pbcfg8(&mut self) -> PBCFG8_W {
        PBCFG8_W { w: self }
    }
    #[doc = "Bits 18:19 - PBCFG9"]
    #[inline(always)]
    pub fn pbcfg9(&mut self) -> PBCFG9_W {
        PBCFG9_W { w: self }
    }
    #[doc = "Bits 20:21 - PBCFG10"]
    #[inline(always)]
    pub fn pbcfg10(&mut self) -> PBCFG10_W {
        PBCFG10_W { w: self }
    }
    #[doc = "Bits 22:23 - PBCFG11"]
    #[inline(always)]
    pub fn pbcfg11(&mut self) -> PBCFG11_W {
        PBCFG11_W { w: self }
    }
    #[doc = "Bits 24:25 - PBCFG12"]
    #[inline(always)]
    pub fn pbcfg12(&mut self) -> PBCFG12_W {
        PBCFG12_W { w: self }
    }
    #[doc = "Bits 26:27 - PBCFG13"]
    #[inline(always)]
    pub fn pbcfg13(&mut self) -> PBCFG13_W {
        PBCFG13_W { w: self }
    }
    #[doc = "Bits 28:29 - PBCFG14"]
    #[inline(always)]
    pub fn pbcfg14(&mut self) -> PBCFG14_W {
        PBCFG14_W { w: self }
    }
    #[doc = "Bits 30:31 - PBCFG15"]
    #[inline(always)]
    pub fn pbcfg15(&mut self) -> PBCFG15_W {
        PBCFG15_W { w: self }
    }
}

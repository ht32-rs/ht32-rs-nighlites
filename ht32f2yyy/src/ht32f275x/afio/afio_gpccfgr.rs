#[doc = "Reader of register AFIO_GPCCFGR"]
pub type R = crate::R<u32, super::AFIO_GPCCFGR>;
#[doc = "Writer for register AFIO_GPCCFGR"]
pub type W = crate::W<u32, super::AFIO_GPCCFGR>;
#[doc = "Register AFIO_GPCCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFIO_GPCCFGR {
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
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
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
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
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
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
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
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
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
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
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
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
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
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
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
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
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
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
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PCCFG0"]
    #[inline(always)]
    pub fn pccfg0(&self) -> PCCFG0_R {
        PCCFG0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - PCCFG1"]
    #[inline(always)]
    pub fn pccfg1(&self) -> PCCFG1_R {
        PCCFG1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - PCCFG2"]
    #[inline(always)]
    pub fn pccfg2(&self) -> PCCFG2_R {
        PCCFG2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - PCCFG3"]
    #[inline(always)]
    pub fn pccfg3(&self) -> PCCFG3_R {
        PCCFG3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - PCCFG4"]
    #[inline(always)]
    pub fn pccfg4(&self) -> PCCFG4_R {
        PCCFG4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - PCCFG5"]
    #[inline(always)]
    pub fn pccfg5(&self) -> PCCFG5_R {
        PCCFG5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - PCCFG6"]
    #[inline(always)]
    pub fn pccfg6(&self) -> PCCFG6_R {
        PCCFG6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - PCCFG7"]
    #[inline(always)]
    pub fn pccfg7(&self) -> PCCFG7_R {
        PCCFG7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - PCCFG8"]
    #[inline(always)]
    pub fn pccfg8(&self) -> PCCFG8_R {
        PCCFG8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - PCCFG9"]
    #[inline(always)]
    pub fn pccfg9(&self) -> PCCFG9_R {
        PCCFG9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - PCCFG10"]
    #[inline(always)]
    pub fn pccfg10(&self) -> PCCFG10_R {
        PCCFG10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - PCCFG11"]
    #[inline(always)]
    pub fn pccfg11(&self) -> PCCFG11_R {
        PCCFG11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - PCCFG12"]
    #[inline(always)]
    pub fn pccfg12(&self) -> PCCFG12_R {
        PCCFG12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - PCCFG13"]
    #[inline(always)]
    pub fn pccfg13(&self) -> PCCFG13_R {
        PCCFG13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - PCCFG14"]
    #[inline(always)]
    pub fn pccfg14(&self) -> PCCFG14_R {
        PCCFG14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - PCCFG15"]
    #[inline(always)]
    pub fn pccfg15(&self) -> PCCFG15_R {
        PCCFG15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PCCFG0"]
    #[inline(always)]
    pub fn pccfg0(&mut self) -> PCCFG0_W {
        PCCFG0_W { w: self }
    }
    #[doc = "Bits 2:3 - PCCFG1"]
    #[inline(always)]
    pub fn pccfg1(&mut self) -> PCCFG1_W {
        PCCFG1_W { w: self }
    }
    #[doc = "Bits 4:5 - PCCFG2"]
    #[inline(always)]
    pub fn pccfg2(&mut self) -> PCCFG2_W {
        PCCFG2_W { w: self }
    }
    #[doc = "Bits 6:7 - PCCFG3"]
    #[inline(always)]
    pub fn pccfg3(&mut self) -> PCCFG3_W {
        PCCFG3_W { w: self }
    }
    #[doc = "Bits 8:9 - PCCFG4"]
    #[inline(always)]
    pub fn pccfg4(&mut self) -> PCCFG4_W {
        PCCFG4_W { w: self }
    }
    #[doc = "Bits 10:11 - PCCFG5"]
    #[inline(always)]
    pub fn pccfg5(&mut self) -> PCCFG5_W {
        PCCFG5_W { w: self }
    }
    #[doc = "Bits 12:13 - PCCFG6"]
    #[inline(always)]
    pub fn pccfg6(&mut self) -> PCCFG6_W {
        PCCFG6_W { w: self }
    }
    #[doc = "Bits 14:15 - PCCFG7"]
    #[inline(always)]
    pub fn pccfg7(&mut self) -> PCCFG7_W {
        PCCFG7_W { w: self }
    }
    #[doc = "Bits 16:17 - PCCFG8"]
    #[inline(always)]
    pub fn pccfg8(&mut self) -> PCCFG8_W {
        PCCFG8_W { w: self }
    }
    #[doc = "Bits 18:19 - PCCFG9"]
    #[inline(always)]
    pub fn pccfg9(&mut self) -> PCCFG9_W {
        PCCFG9_W { w: self }
    }
    #[doc = "Bits 20:21 - PCCFG10"]
    #[inline(always)]
    pub fn pccfg10(&mut self) -> PCCFG10_W {
        PCCFG10_W { w: self }
    }
    #[doc = "Bits 22:23 - PCCFG11"]
    #[inline(always)]
    pub fn pccfg11(&mut self) -> PCCFG11_W {
        PCCFG11_W { w: self }
    }
    #[doc = "Bits 24:25 - PCCFG12"]
    #[inline(always)]
    pub fn pccfg12(&mut self) -> PCCFG12_W {
        PCCFG12_W { w: self }
    }
    #[doc = "Bits 26:27 - PCCFG13"]
    #[inline(always)]
    pub fn pccfg13(&mut self) -> PCCFG13_W {
        PCCFG13_W { w: self }
    }
    #[doc = "Bits 28:29 - PCCFG14"]
    #[inline(always)]
    pub fn pccfg14(&mut self) -> PCCFG14_W {
        PCCFG14_W { w: self }
    }
    #[doc = "Bits 30:31 - PCCFG15"]
    #[inline(always)]
    pub fn pccfg15(&mut self) -> PCCFG15_W {
        PCCFG15_W { w: self }
    }
}

#[doc = "Reader of register CKCU_APBPCSR1"]
pub type R = crate::R<u32, super::CKCU_APBPCSR1>;
#[doc = "Writer for register CKCU_APBPCSR1"]
pub type W = crate::W<u32, super::CKCU_APBPCSR1>;
#[doc = "Register CKCU_APBPCSR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CKCU_APBPCSR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AFIOPCLK`"]
pub type AFIOPCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AFIOPCLK`"]
pub struct AFIOPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> AFIOPCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `EXTIPCLK`"]
pub type EXTIPCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTIPCLK`"]
pub struct EXTIPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `ADCCPCLK`"]
pub type ADCCPCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCCPCLK`"]
pub struct ADCCPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCCPCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CMPPCLK`"]
pub type CMPPCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPPCLK`"]
pub struct CMPPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPPCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `WDTRPCLK`"]
pub type WDTRPCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WDTRPCLK`"]
pub struct WDTRPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTRPCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `BKPRPCLK`"]
pub type BKPRPCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BKPRPCLK`"]
pub struct BKPRPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPRPCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `SCIPCLK`"]
pub type SCIPCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCIPCLK`"]
pub struct SCIPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SCIPCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `I2SPCLK`"]
pub type I2SPCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2SPCLK`"]
pub struct I2SPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SPCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - AFIOPCLK"]
    #[inline(always)]
    pub fn afiopclk(&self) -> AFIOPCLK_R {
        AFIOPCLK_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - EXTIPCLK"]
    #[inline(always)]
    pub fn extipclk(&self) -> EXTIPCLK_R {
        EXTIPCLK_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - ADCCPCLK"]
    #[inline(always)]
    pub fn adccpclk(&self) -> ADCCPCLK_R {
        ADCCPCLK_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - CMPPCLK"]
    #[inline(always)]
    pub fn cmppclk(&self) -> CMPPCLK_R {
        CMPPCLK_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - WDTRPCLK"]
    #[inline(always)]
    pub fn wdtrpclk(&self) -> WDTRPCLK_R {
        WDTRPCLK_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - BKPRPCLK"]
    #[inline(always)]
    pub fn bkprpclk(&self) -> BKPRPCLK_R {
        BKPRPCLK_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - SCIPCLK"]
    #[inline(always)]
    pub fn scipclk(&self) -> SCIPCLK_R {
        SCIPCLK_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - I2SPCLK"]
    #[inline(always)]
    pub fn i2spclk(&self) -> I2SPCLK_R {
        I2SPCLK_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AFIOPCLK"]
    #[inline(always)]
    pub fn afiopclk(&mut self) -> AFIOPCLK_W {
        AFIOPCLK_W { w: self }
    }
    #[doc = "Bits 2:3 - EXTIPCLK"]
    #[inline(always)]
    pub fn extipclk(&mut self) -> EXTIPCLK_W {
        EXTIPCLK_W { w: self }
    }
    #[doc = "Bits 4:5 - ADCCPCLK"]
    #[inline(always)]
    pub fn adccpclk(&mut self) -> ADCCPCLK_W {
        ADCCPCLK_W { w: self }
    }
    #[doc = "Bits 8:9 - CMPPCLK"]
    #[inline(always)]
    pub fn cmppclk(&mut self) -> CMPPCLK_W {
        CMPPCLK_W { w: self }
    }
    #[doc = "Bits 12:13 - WDTRPCLK"]
    #[inline(always)]
    pub fn wdtrpclk(&mut self) -> WDTRPCLK_W {
        WDTRPCLK_W { w: self }
    }
    #[doc = "Bits 14:15 - BKPRPCLK"]
    #[inline(always)]
    pub fn bkprpclk(&mut self) -> BKPRPCLK_W {
        BKPRPCLK_W { w: self }
    }
    #[doc = "Bits 16:17 - SCIPCLK"]
    #[inline(always)]
    pub fn scipclk(&mut self) -> SCIPCLK_W {
        SCIPCLK_W { w: self }
    }
    #[doc = "Bits 20:21 - I2SPCLK"]
    #[inline(always)]
    pub fn i2spclk(&mut self) -> I2SPCLK_W {
        I2SPCLK_W { w: self }
    }
}

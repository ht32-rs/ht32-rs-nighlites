#[doc = "Reader of register GPIOD_DRVR"]
pub type R = crate::R<u32, super::GPIOD_DRVR>;
#[doc = "Writer for register GPIOD_DRVR"]
pub type W = crate::W<u32, super::GPIOD_DRVR>;
#[doc = "Register GPIOD_DRVR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOD_DRVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DV0`"]
pub type DV0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV0`"]
pub struct DV0_W<'a> {
    w: &'a mut W,
}
impl<'a> DV0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DV1`"]
pub type DV1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV1`"]
pub struct DV1_W<'a> {
    w: &'a mut W,
}
impl<'a> DV1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `DV2`"]
pub type DV2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV2`"]
pub struct DV2_W<'a> {
    w: &'a mut W,
}
impl<'a> DV2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - DV0"]
    #[inline(always)]
    pub fn dv0(&self) -> DV0_R {
        DV0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - DV1"]
    #[inline(always)]
    pub fn dv1(&self) -> DV1_R {
        DV1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - DV2"]
    #[inline(always)]
    pub fn dv2(&self) -> DV2_R {
        DV2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DV0"]
    #[inline(always)]
    pub fn dv0(&mut self) -> DV0_W {
        DV0_W { w: self }
    }
    #[doc = "Bits 2:3 - DV1"]
    #[inline(always)]
    pub fn dv1(&mut self) -> DV1_W {
        DV1_W { w: self }
    }
    #[doc = "Bits 4:5 - DV2"]
    #[inline(always)]
    pub fn dv2(&mut self) -> DV2_W {
        DV2_W { w: self }
    }
}

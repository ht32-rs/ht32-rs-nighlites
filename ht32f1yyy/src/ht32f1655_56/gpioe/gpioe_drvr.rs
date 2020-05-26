#[doc = "Reader of register GPIOE_DRVR"]
pub type R = crate::R<u32, super::GPIOE_DRVR>;
#[doc = "Writer for register GPIOE_DRVR"]
pub type W = crate::W<u32, super::GPIOE_DRVR>;
#[doc = "Register GPIOE_DRVR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOE_DRVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DV8`"]
pub type DV8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV8`"]
pub struct DV8_W<'a> {
    w: &'a mut W,
}
impl<'a> DV8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DV9`"]
pub type DV9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV9`"]
pub struct DV9_W<'a> {
    w: &'a mut W,
}
impl<'a> DV9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DV10`"]
pub type DV10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV10`"]
pub struct DV10_W<'a> {
    w: &'a mut W,
}
impl<'a> DV10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DV11`"]
pub type DV11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV11`"]
pub struct DV11_W<'a> {
    w: &'a mut W,
}
impl<'a> DV11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DV12`"]
pub type DV12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV12`"]
pub struct DV12_W<'a> {
    w: &'a mut W,
}
impl<'a> DV12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - DV8"]
    #[inline(always)]
    pub fn dv8(&self) -> DV8_R {
        DV8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DV9"]
    #[inline(always)]
    pub fn dv9(&self) -> DV9_R {
        DV9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DV10"]
    #[inline(always)]
    pub fn dv10(&self) -> DV10_R {
        DV10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DV11"]
    #[inline(always)]
    pub fn dv11(&self) -> DV11_R {
        DV11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DV12"]
    #[inline(always)]
    pub fn dv12(&self) -> DV12_R {
        DV12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - DV8"]
    #[inline(always)]
    pub fn dv8(&mut self) -> DV8_W {
        DV8_W { w: self }
    }
    #[doc = "Bit 9 - DV9"]
    #[inline(always)]
    pub fn dv9(&mut self) -> DV9_W {
        DV9_W { w: self }
    }
    #[doc = "Bit 10 - DV10"]
    #[inline(always)]
    pub fn dv10(&mut self) -> DV10_W {
        DV10_W { w: self }
    }
    #[doc = "Bit 11 - DV11"]
    #[inline(always)]
    pub fn dv11(&mut self) -> DV11_W {
        DV11_W { w: self }
    }
    #[doc = "Bit 12 - DV12"]
    #[inline(always)]
    pub fn dv12(&mut self) -> DV12_W {
        DV12_W { w: self }
    }
}

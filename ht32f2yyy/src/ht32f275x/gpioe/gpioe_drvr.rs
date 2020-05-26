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
#[doc = "Reader of field `DV5`"]
pub type DV5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV5`"]
pub struct DV5_W<'a> {
    w: &'a mut W,
}
impl<'a> DV5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DV6`"]
pub type DV6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV6`"]
pub struct DV6_W<'a> {
    w: &'a mut W,
}
impl<'a> DV6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DV7`"]
pub type DV7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DV7`"]
pub struct DV7_W<'a> {
    w: &'a mut W,
}
impl<'a> DV7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
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
impl R {
    #[doc = "Bit 5 - DV5"]
    #[inline(always)]
    pub fn dv5(&self) -> DV5_R {
        DV5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DV6"]
    #[inline(always)]
    pub fn dv6(&self) -> DV6_R {
        DV6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DV7"]
    #[inline(always)]
    pub fn dv7(&self) -> DV7_R {
        DV7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
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
}
impl W {
    #[doc = "Bit 5 - DV5"]
    #[inline(always)]
    pub fn dv5(&mut self) -> DV5_W {
        DV5_W { w: self }
    }
    #[doc = "Bit 6 - DV6"]
    #[inline(always)]
    pub fn dv6(&mut self) -> DV6_W {
        DV6_W { w: self }
    }
    #[doc = "Bit 7 - DV7"]
    #[inline(always)]
    pub fn dv7(&mut self) -> DV7_W {
        DV7_W { w: self }
    }
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
}

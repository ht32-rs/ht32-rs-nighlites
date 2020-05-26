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
impl R {
    #[doc = "Bit 6 - DV6"]
    #[inline(always)]
    pub fn dv6(&self) -> DV6_R {
        DV6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - DV6"]
    #[inline(always)]
    pub fn dv6(&mut self) -> DV6_W {
        DV6_W { w: self }
    }
}

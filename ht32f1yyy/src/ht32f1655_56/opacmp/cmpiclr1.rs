#[doc = "Reader of register CMPICLR1"]
pub type R = crate::R<u32, super::CMPICLR1>;
#[doc = "Writer for register CMPICLR1"]
pub type W = crate::W<u32, super::CMPICLR1>;
#[doc = "Register CMPICLR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPICLR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CF1ICLR`"]
pub type CF1ICLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CF1ICLR`"]
pub struct CF1ICLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CF1ICLR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `CR1ICLR`"]
pub type CR1ICLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR1ICLR`"]
pub struct CR1ICLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR1ICLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CF1ICLR"]
    #[inline(always)]
    pub fn cf1iclr(&self) -> CF1ICLR_R {
        CF1ICLR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CR1ICLR"]
    #[inline(always)]
    pub fn cr1iclr(&self) -> CR1ICLR_R {
        CR1ICLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CF1ICLR"]
    #[inline(always)]
    pub fn cf1iclr(&mut self) -> CF1ICLR_W {
        CF1ICLR_W { w: self }
    }
    #[doc = "Bit 1 - CR1ICLR"]
    #[inline(always)]
    pub fn cr1iclr(&mut self) -> CR1ICLR_W {
        CR1ICLR_W { w: self }
    }
}

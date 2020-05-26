#[doc = "Reader of register CMPICLR0"]
pub type R = crate::R<u32, super::CMPICLR0>;
#[doc = "Writer for register CMPICLR0"]
pub type W = crate::W<u32, super::CMPICLR0>;
#[doc = "Register CMPICLR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPICLR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CF0ICLR`"]
pub type CF0ICLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CF0ICLR`"]
pub struct CF0ICLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CF0ICLR_W<'a> {
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
#[doc = "Reader of field `CR0ICLR`"]
pub type CR0ICLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR0ICLR`"]
pub struct CR0ICLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR0ICLR_W<'a> {
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
    #[doc = "Bit 0 - CF0ICLR"]
    #[inline(always)]
    pub fn cf0iclr(&self) -> CF0ICLR_R {
        CF0ICLR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CR0ICLR"]
    #[inline(always)]
    pub fn cr0iclr(&self) -> CR0ICLR_R {
        CR0ICLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CF0ICLR"]
    #[inline(always)]
    pub fn cf0iclr(&mut self) -> CF0ICLR_W {
        CF0ICLR_W { w: self }
    }
    #[doc = "Bit 1 - CR0ICLR"]
    #[inline(always)]
    pub fn cr0iclr(&mut self) -> CR0ICLR_W {
        CR0ICLR_W { w: self }
    }
}

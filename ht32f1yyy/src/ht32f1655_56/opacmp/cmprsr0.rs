#[doc = "Reader of register CMPRSR0"]
pub type R = crate::R<u32, super::CMPRSR0>;
#[doc = "Writer for register CMPRSR0"]
pub type W = crate::W<u32, super::CMPRSR0>;
#[doc = "Register CMPRSR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPRSR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CF0RAW`"]
pub type CF0RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CF0RAW`"]
pub struct CF0RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CF0RAW_W<'a> {
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
#[doc = "Reader of field `CR0RAW`"]
pub type CR0RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR0RAW`"]
pub struct CR0RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CR0RAW_W<'a> {
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
    #[doc = "Bit 0 - CF0RAW"]
    #[inline(always)]
    pub fn cf0raw(&self) -> CF0RAW_R {
        CF0RAW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CR0RAW"]
    #[inline(always)]
    pub fn cr0raw(&self) -> CR0RAW_R {
        CR0RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CF0RAW"]
    #[inline(always)]
    pub fn cf0raw(&mut self) -> CF0RAW_W {
        CF0RAW_W { w: self }
    }
    #[doc = "Bit 1 - CR0RAW"]
    #[inline(always)]
    pub fn cr0raw(&mut self) -> CR0RAW_W {
        CR0RAW_W { w: self }
    }
}

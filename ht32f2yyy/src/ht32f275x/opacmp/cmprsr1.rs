#[doc = "Reader of register CMPRSR1"]
pub type R = crate::R<u32, super::CMPRSR1>;
#[doc = "Writer for register CMPRSR1"]
pub type W = crate::W<u32, super::CMPRSR1>;
#[doc = "Register CMPRSR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPRSR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CF1RAW`"]
pub type CF1RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CF1RAW`"]
pub struct CF1RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CF1RAW_W<'a> {
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
#[doc = "Reader of field `CR1RAW`"]
pub type CR1RAW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR1RAW`"]
pub struct CR1RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> CR1RAW_W<'a> {
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
    #[doc = "Bit 0 - CF1RAW"]
    #[inline(always)]
    pub fn cf1raw(&self) -> CF1RAW_R {
        CF1RAW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CR1RAW"]
    #[inline(always)]
    pub fn cr1raw(&self) -> CR1RAW_R {
        CR1RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CF1RAW"]
    #[inline(always)]
    pub fn cf1raw(&mut self) -> CF1RAW_W {
        CF1RAW_W { w: self }
    }
    #[doc = "Bit 1 - CR1RAW"]
    #[inline(always)]
    pub fn cr1raw(&mut self) -> CR1RAW_W {
        CR1RAW_W { w: self }
    }
}

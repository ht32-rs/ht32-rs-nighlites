#[doc = "Reader of register CMPIER1"]
pub type R = crate::R<u32, super::CMPIER1>;
#[doc = "Writer for register CMPIER1"]
pub type W = crate::W<u32, super::CMPIER1>;
#[doc = "Register CMPIER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPIER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CF1IEN`"]
pub type CF1IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CF1IEN`"]
pub struct CF1IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CF1IEN_W<'a> {
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
#[doc = "Reader of field `CR1IEN`"]
pub type CR1IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR1IEN`"]
pub struct CR1IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR1IEN_W<'a> {
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
    #[doc = "Bit 0 - CF1IEN"]
    #[inline(always)]
    pub fn cf1ien(&self) -> CF1IEN_R {
        CF1IEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CR1IEN"]
    #[inline(always)]
    pub fn cr1ien(&self) -> CR1IEN_R {
        CR1IEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CF1IEN"]
    #[inline(always)]
    pub fn cf1ien(&mut self) -> CF1IEN_W {
        CF1IEN_W { w: self }
    }
    #[doc = "Bit 1 - CR1IEN"]
    #[inline(always)]
    pub fn cr1ien(&mut self) -> CR1IEN_W {
        CR1IEN_W { w: self }
    }
}

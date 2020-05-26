#[doc = "Reader of register CMPISR1"]
pub type R = crate::R<u32, super::CMPISR1>;
#[doc = "Writer for register CMPISR1"]
pub type W = crate::W<u32, super::CMPISR1>;
#[doc = "Register CMPISR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPISR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CF1IS`"]
pub type CF1IS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CF1IS`"]
pub struct CF1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> CF1IS_W<'a> {
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
#[doc = "Reader of field `CR1IS`"]
pub type CR1IS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR1IS`"]
pub struct CR1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> CR1IS_W<'a> {
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
    #[doc = "Bit 0 - CF1IS"]
    #[inline(always)]
    pub fn cf1is(&self) -> CF1IS_R {
        CF1IS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CR1IS"]
    #[inline(always)]
    pub fn cr1is(&self) -> CR1IS_R {
        CR1IS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CF1IS"]
    #[inline(always)]
    pub fn cf1is(&mut self) -> CF1IS_W {
        CF1IS_W { w: self }
    }
    #[doc = "Bit 1 - CR1IS"]
    #[inline(always)]
    pub fn cr1is(&mut self) -> CR1IS_W {
        CR1IS_W { w: self }
    }
}

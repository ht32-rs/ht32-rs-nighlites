#[doc = "Reader of register CMPISR0"]
pub type R = crate::R<u32, super::CMPISR0>;
#[doc = "Writer for register CMPISR0"]
pub type W = crate::W<u32, super::CMPISR0>;
#[doc = "Register CMPISR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPISR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CF0IS`"]
pub type CF0IS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CF0IS`"]
pub struct CF0IS_W<'a> {
    w: &'a mut W,
}
impl<'a> CF0IS_W<'a> {
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
#[doc = "Reader of field `CR0IS`"]
pub type CR0IS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR0IS`"]
pub struct CR0IS_W<'a> {
    w: &'a mut W,
}
impl<'a> CR0IS_W<'a> {
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
    #[doc = "Bit 0 - CF0IS"]
    #[inline(always)]
    pub fn cf0is(&self) -> CF0IS_R {
        CF0IS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CR0IS"]
    #[inline(always)]
    pub fn cr0is(&self) -> CR0IS_R {
        CR0IS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CF0IS"]
    #[inline(always)]
    pub fn cf0is(&mut self) -> CF0IS_W {
        CF0IS_W { w: self }
    }
    #[doc = "Bit 1 - CR0IS"]
    #[inline(always)]
    pub fn cr0is(&mut self) -> CR0IS_W {
        CR0IS_W { w: self }
    }
}

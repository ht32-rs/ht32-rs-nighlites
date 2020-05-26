#[doc = "Reader of register CMPIER0"]
pub type R = crate::R<u32, super::CMPIER0>;
#[doc = "Writer for register CMPIER0"]
pub type W = crate::W<u32, super::CMPIER0>;
#[doc = "Register CMPIER0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPIER0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CF0IEN`"]
pub type CF0IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CF0IEN`"]
pub struct CF0IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CF0IEN_W<'a> {
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
#[doc = "Reader of field `CR0IEN`"]
pub type CR0IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR0IEN`"]
pub struct CR0IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR0IEN_W<'a> {
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
    #[doc = "Bit 0 - CF0IEN"]
    #[inline(always)]
    pub fn cf0ien(&self) -> CF0IEN_R {
        CF0IEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CR0IEN"]
    #[inline(always)]
    pub fn cr0ien(&self) -> CR0IEN_R {
        CR0IEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CF0IEN"]
    #[inline(always)]
    pub fn cf0ien(&mut self) -> CF0IEN_W {
        CF0IEN_W { w: self }
    }
    #[doc = "Bit 1 - CR0IEN"]
    #[inline(always)]
    pub fn cr0ien(&mut self) -> CR0IEN_W {
        CR0IEN_W { w: self }
    }
}

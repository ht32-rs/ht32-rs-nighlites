#[doc = "Reader of register EBI_SR"]
pub type R = crate::R<u32, super::EBI_SR>;
#[doc = "Writer for register EBI_SR"]
pub type W = crate::W<u32, super::EBI_SR>;
#[doc = "Register EBI_SR `reset()`'s with value 0"]
impl crate::ResetValue for super::EBI_SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EBIBUSY`"]
pub type EBIBUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBIBUSY`"]
pub struct EBIBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> EBIBUSY_W<'a> {
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
#[doc = "Reader of field `EBIARDY`"]
pub type EBIARDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBIARDY`"]
pub struct EBIARDY_W<'a> {
    w: &'a mut W,
}
impl<'a> EBIARDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `EBISMRST`"]
pub type EBISMRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBISMRST`"]
pub struct EBISMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> EBISMRST_W<'a> {
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
impl R {
    #[doc = "Bit 0 - EBIBUSY"]
    #[inline(always)]
    pub fn ebibusy(&self) -> EBIBUSY_R {
        EBIBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - EBIARDY"]
    #[inline(always)]
    pub fn ebiardy(&self) -> EBIARDY_R {
        EBIARDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EBISMRST"]
    #[inline(always)]
    pub fn ebismrst(&self) -> EBISMRST_R {
        EBISMRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EBIBUSY"]
    #[inline(always)]
    pub fn ebibusy(&mut self) -> EBIBUSY_W {
        EBIBUSY_W { w: self }
    }
    #[doc = "Bit 4 - EBIARDY"]
    #[inline(always)]
    pub fn ebiardy(&mut self) -> EBIARDY_W {
        EBIARDY_W { w: self }
    }
    #[doc = "Bit 8 - EBISMRST"]
    #[inline(always)]
    pub fn ebismrst(&mut self) -> EBISMRST_W {
        EBISMRST_W { w: self }
    }
}

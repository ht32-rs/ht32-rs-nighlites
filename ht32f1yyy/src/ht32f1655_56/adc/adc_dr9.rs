#[doc = "Reader of register ADC_DR9"]
pub type R = crate::R<u32, super::ADC_DR9>;
#[doc = "Writer for register ADC_DR9"]
pub type W = crate::W<u32, super::ADC_DR9>;
#[doc = "Register ADC_DR9 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_DR9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD9`"]
pub type ADD9_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD9`"]
pub struct ADD9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADVLD9`"]
pub type ADVLD9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVLD9`"]
pub struct ADVLD9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVLD9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ADD9"]
    #[inline(always)]
    pub fn add9(&self) -> ADD9_R {
        ADD9_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD9"]
    #[inline(always)]
    pub fn advld9(&self) -> ADVLD9_R {
        ADVLD9_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD9"]
    #[inline(always)]
    pub fn add9(&mut self) -> ADD9_W {
        ADD9_W { w: self }
    }
    #[doc = "Bit 31 - ADVLD9"]
    #[inline(always)]
    pub fn advld9(&mut self) -> ADVLD9_W {
        ADVLD9_W { w: self }
    }
}

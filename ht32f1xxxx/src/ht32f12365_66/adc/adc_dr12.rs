#[doc = "Reader of register ADC_DR12"]
pub type R = crate::R<u32, super::ADC_DR12>;
#[doc = "Writer for register ADC_DR12"]
pub type W = crate::W<u32, super::ADC_DR12>;
#[doc = "Register ADC_DR12 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_DR12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD12`"]
pub type ADD12_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD12`"]
pub struct ADD12_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADVLD12`"]
pub type ADVLD12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVLD12`"]
pub struct ADVLD12_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVLD12_W<'a> {
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
    #[doc = "Bits 0:15 - ADD12"]
    #[inline(always)]
    pub fn add12(&self) -> ADD12_R {
        ADD12_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD12"]
    #[inline(always)]
    pub fn advld12(&self) -> ADVLD12_R {
        ADVLD12_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD12"]
    #[inline(always)]
    pub fn add12(&mut self) -> ADD12_W {
        ADD12_W { w: self }
    }
    #[doc = "Bit 31 - ADVLD12"]
    #[inline(always)]
    pub fn advld12(&mut self) -> ADVLD12_W {
        ADVLD12_W { w: self }
    }
}

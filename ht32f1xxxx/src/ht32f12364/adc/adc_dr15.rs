#[doc = "Reader of register ADC_DR15"]
pub type R = crate::R<u32, super::ADC_DR15>;
#[doc = "Writer for register ADC_DR15"]
pub type W = crate::W<u32, super::ADC_DR15>;
#[doc = "Register ADC_DR15 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_DR15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD15`"]
pub type ADD15_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD15`"]
pub struct ADD15_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADVLD15`"]
pub type ADVLD15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVLD15`"]
pub struct ADVLD15_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVLD15_W<'a> {
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
    #[doc = "Bits 0:15 - ADD15"]
    #[inline(always)]
    pub fn add15(&self) -> ADD15_R {
        ADD15_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD15"]
    #[inline(always)]
    pub fn advld15(&self) -> ADVLD15_R {
        ADVLD15_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD15"]
    #[inline(always)]
    pub fn add15(&mut self) -> ADD15_W {
        ADD15_W { w: self }
    }
    #[doc = "Bit 31 - ADVLD15"]
    #[inline(always)]
    pub fn advld15(&mut self) -> ADVLD15_W {
        ADVLD15_W { w: self }
    }
}

#[doc = "Reader of register ADC_DR8"]
pub type R = crate::R<u32, super::ADC_DR8>;
#[doc = "Writer for register ADC_DR8"]
pub type W = crate::W<u32, super::ADC_DR8>;
#[doc = "Register ADC_DR8 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_DR8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD8`"]
pub type ADD8_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD8`"]
pub struct ADD8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADVLD8`"]
pub type ADVLD8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVLD8`"]
pub struct ADVLD8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVLD8_W<'a> {
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
    #[doc = "Bits 0:15 - ADD8"]
    #[inline(always)]
    pub fn add8(&self) -> ADD8_R {
        ADD8_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD8"]
    #[inline(always)]
    pub fn advld8(&self) -> ADVLD8_R {
        ADVLD8_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD8"]
    #[inline(always)]
    pub fn add8(&mut self) -> ADD8_W {
        ADD8_W { w: self }
    }
    #[doc = "Bit 31 - ADVLD8"]
    #[inline(always)]
    pub fn advld8(&mut self) -> ADVLD8_W {
        ADVLD8_W { w: self }
    }
}

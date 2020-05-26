#[doc = "Reader of register ADC_DR10"]
pub type R = crate::R<u32, super::ADC_DR10>;
#[doc = "Writer for register ADC_DR10"]
pub type W = crate::W<u32, super::ADC_DR10>;
#[doc = "Register ADC_DR10 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_DR10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD10`"]
pub type ADD10_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD10`"]
pub struct ADD10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADVLD10`"]
pub type ADVLD10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVLD10`"]
pub struct ADVLD10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVLD10_W<'a> {
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
    #[doc = "Bits 0:15 - ADD10"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD10"]
    #[inline(always)]
    pub fn advld10(&self) -> ADVLD10_R {
        ADVLD10_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD10"]
    #[inline(always)]
    pub fn add10(&mut self) -> ADD10_W {
        ADD10_W { w: self }
    }
    #[doc = "Bit 31 - ADVLD10"]
    #[inline(always)]
    pub fn advld10(&mut self) -> ADVLD10_W {
        ADVLD10_W { w: self }
    }
}

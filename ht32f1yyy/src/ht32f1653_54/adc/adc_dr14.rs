#[doc = "Reader of register ADC_DR14"]
pub type R = crate::R<u32, super::ADC_DR14>;
#[doc = "Writer for register ADC_DR14"]
pub type W = crate::W<u32, super::ADC_DR14>;
#[doc = "Register ADC_DR14 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_DR14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD14`"]
pub type ADD14_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD14`"]
pub struct ADD14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADVLD14`"]
pub type ADVLD14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVLD14`"]
pub struct ADVLD14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVLD14_W<'a> {
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
    #[doc = "Bits 0:15 - ADD14"]
    #[inline(always)]
    pub fn add14(&self) -> ADD14_R {
        ADD14_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD14"]
    #[inline(always)]
    pub fn advld14(&self) -> ADVLD14_R {
        ADVLD14_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD14"]
    #[inline(always)]
    pub fn add14(&mut self) -> ADD14_W {
        ADD14_W { w: self }
    }
    #[doc = "Bit 31 - ADVLD14"]
    #[inline(always)]
    pub fn advld14(&mut self) -> ADVLD14_W {
        ADVLD14_W { w: self }
    }
}

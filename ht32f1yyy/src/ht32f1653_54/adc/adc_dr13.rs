#[doc = "Reader of register ADC_DR13"]
pub type R = crate::R<u32, super::ADC_DR13>;
#[doc = "Writer for register ADC_DR13"]
pub type W = crate::W<u32, super::ADC_DR13>;
#[doc = "Register ADC_DR13 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_DR13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD13`"]
pub type ADD13_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD13`"]
pub struct ADD13_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADVLD13`"]
pub type ADVLD13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADVLD13`"]
pub struct ADVLD13_W<'a> {
    w: &'a mut W,
}
impl<'a> ADVLD13_W<'a> {
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
    #[doc = "Bits 0:15 - ADD13"]
    #[inline(always)]
    pub fn add13(&self) -> ADD13_R {
        ADD13_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADVLD13"]
    #[inline(always)]
    pub fn advld13(&self) -> ADVLD13_R {
        ADVLD13_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADD13"]
    #[inline(always)]
    pub fn add13(&mut self) -> ADD13_W {
        ADD13_W { w: self }
    }
    #[doc = "Bit 31 - ADVLD13"]
    #[inline(always)]
    pub fn advld13(&mut self) -> ADVLD13_W {
        ADVLD13_W { w: self }
    }
}

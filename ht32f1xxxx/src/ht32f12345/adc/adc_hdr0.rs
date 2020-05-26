#[doc = "Reader of register ADC_HDR0"]
pub type R = crate::R<u32, super::ADC_HDR0>;
#[doc = "Writer for register ADC_HDR0"]
pub type W = crate::W<u32, super::ADC_HDR0>;
#[doc = "Register ADC_HDR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_HDR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADHD0`"]
pub type ADHD0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADHD0`"]
pub struct ADHD0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADHVLD0`"]
pub type ADHVLD0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADHVLD0`"]
pub struct ADHVLD0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHVLD0_W<'a> {
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
    #[doc = "Bits 0:15 - ADHD0"]
    #[inline(always)]
    pub fn adhd0(&self) -> ADHD0_R {
        ADHD0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADHVLD0"]
    #[inline(always)]
    pub fn adhvld0(&self) -> ADHVLD0_R {
        ADHVLD0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADHD0"]
    #[inline(always)]
    pub fn adhd0(&mut self) -> ADHD0_W {
        ADHD0_W { w: self }
    }
    #[doc = "Bit 31 - ADHVLD0"]
    #[inline(always)]
    pub fn adhvld0(&mut self) -> ADHVLD0_W {
        ADHVLD0_W { w: self }
    }
}

#[doc = "Reader of register ADC_HDR1"]
pub type R = crate::R<u32, super::ADC_HDR1>;
#[doc = "Writer for register ADC_HDR1"]
pub type W = crate::W<u32, super::ADC_HDR1>;
#[doc = "Register ADC_HDR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_HDR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADHD1`"]
pub type ADHD1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADHD1`"]
pub struct ADHD1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADHVLD1`"]
pub type ADHVLD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADHVLD1`"]
pub struct ADHVLD1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHVLD1_W<'a> {
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
    #[doc = "Bits 0:15 - ADHD1"]
    #[inline(always)]
    pub fn adhd1(&self) -> ADHD1_R {
        ADHD1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADHVLD1"]
    #[inline(always)]
    pub fn adhvld1(&self) -> ADHVLD1_R {
        ADHVLD1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADHD1"]
    #[inline(always)]
    pub fn adhd1(&mut self) -> ADHD1_W {
        ADHD1_W { w: self }
    }
    #[doc = "Bit 31 - ADHVLD1"]
    #[inline(always)]
    pub fn adhvld1(&mut self) -> ADHVLD1_W {
        ADHVLD1_W { w: self }
    }
}

#[doc = "Reader of register ADC_HDR2"]
pub type R = crate::R<u32, super::ADC_HDR2>;
#[doc = "Writer for register ADC_HDR2"]
pub type W = crate::W<u32, super::ADC_HDR2>;
#[doc = "Register ADC_HDR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_HDR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADHD2`"]
pub type ADHD2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADHD2`"]
pub struct ADHD2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADHVLD2`"]
pub type ADHVLD2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADHVLD2`"]
pub struct ADHVLD2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHVLD2_W<'a> {
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
    #[doc = "Bits 0:15 - ADHD2"]
    #[inline(always)]
    pub fn adhd2(&self) -> ADHD2_R {
        ADHD2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADHVLD2"]
    #[inline(always)]
    pub fn adhvld2(&self) -> ADHVLD2_R {
        ADHVLD2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADHD2"]
    #[inline(always)]
    pub fn adhd2(&mut self) -> ADHD2_W {
        ADHD2_W { w: self }
    }
    #[doc = "Bit 31 - ADHVLD2"]
    #[inline(always)]
    pub fn adhvld2(&mut self) -> ADHVLD2_W {
        ADHVLD2_W { w: self }
    }
}

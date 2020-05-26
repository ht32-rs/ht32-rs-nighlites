#[doc = "Reader of register ADC_HDR3"]
pub type R = crate::R<u32, super::ADC_HDR3>;
#[doc = "Writer for register ADC_HDR3"]
pub type W = crate::W<u32, super::ADC_HDR3>;
#[doc = "Register ADC_HDR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_HDR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADHD3`"]
pub type ADHD3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADHD3`"]
pub struct ADHD3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHD3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ADHVLD3`"]
pub type ADHVLD3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADHVLD3`"]
pub struct ADHVLD3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHVLD3_W<'a> {
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
    #[doc = "Bits 0:15 - ADHD3"]
    #[inline(always)]
    pub fn adhd3(&self) -> ADHD3_R {
        ADHD3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADHVLD3"]
    #[inline(always)]
    pub fn adhvld3(&self) -> ADHVLD3_R {
        ADHVLD3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADHD3"]
    #[inline(always)]
    pub fn adhd3(&mut self) -> ADHD3_W {
        ADHD3_W { w: self }
    }
    #[doc = "Bit 31 - ADHVLD3"]
    #[inline(always)]
    pub fn adhvld3(&mut self) -> ADHVLD3_W {
        ADHVLD3_W { w: self }
    }
}

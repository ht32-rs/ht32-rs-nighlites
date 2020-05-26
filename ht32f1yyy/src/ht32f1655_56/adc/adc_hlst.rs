#[doc = "Reader of register ADC_HLST"]
pub type R = crate::R<u32, super::ADC_HLST>;
#[doc = "Writer for register ADC_HLST"]
pub type W = crate::W<u32, super::ADC_HLST>;
#[doc = "Register ADC_HLST `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_HLST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADHSEQ0`"]
pub type ADHSEQ0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADHSEQ0`"]
pub struct ADHSEQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHSEQ0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ADHSEQ1`"]
pub type ADHSEQ1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADHSEQ1`"]
pub struct ADHSEQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHSEQ1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADHSEQ2`"]
pub type ADHSEQ2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADHSEQ2`"]
pub struct ADHSEQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHSEQ2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADHSEQ3`"]
pub type ADHSEQ3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADHSEQ3`"]
pub struct ADHSEQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHSEQ3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - ADHSEQ0"]
    #[inline(always)]
    pub fn adhseq0(&self) -> ADHSEQ0_R {
        ADHSEQ0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - ADHSEQ1"]
    #[inline(always)]
    pub fn adhseq1(&self) -> ADHSEQ1_R {
        ADHSEQ1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADHSEQ2"]
    #[inline(always)]
    pub fn adhseq2(&self) -> ADHSEQ2_R {
        ADHSEQ2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADHSEQ3"]
    #[inline(always)]
    pub fn adhseq3(&self) -> ADHSEQ3_R {
        ADHSEQ3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADHSEQ0"]
    #[inline(always)]
    pub fn adhseq0(&mut self) -> ADHSEQ0_W {
        ADHSEQ0_W { w: self }
    }
    #[doc = "Bits 8:12 - ADHSEQ1"]
    #[inline(always)]
    pub fn adhseq1(&mut self) -> ADHSEQ1_W {
        ADHSEQ1_W { w: self }
    }
    #[doc = "Bits 16:20 - ADHSEQ2"]
    #[inline(always)]
    pub fn adhseq2(&mut self) -> ADHSEQ2_W {
        ADHSEQ2_W { w: self }
    }
    #[doc = "Bits 24:28 - ADHSEQ3"]
    #[inline(always)]
    pub fn adhseq3(&mut self) -> ADHSEQ3_W {
        ADHSEQ3_W { w: self }
    }
}

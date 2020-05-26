#[doc = "Reader of register ADC_LST2"]
pub type R = crate::R<u32, super::ADC_LST2>;
#[doc = "Writer for register ADC_LST2"]
pub type W = crate::W<u32, super::ADC_LST2>;
#[doc = "Register ADC_LST2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_LST2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADSEQ8`"]
pub type ADSEQ8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQ8`"]
pub struct ADSEQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQ8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ADSEQ9`"]
pub type ADSEQ9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQ9`"]
pub struct ADSEQ9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQ9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADSEQ10`"]
pub type ADSEQ10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQ10`"]
pub struct ADSEQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQ10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADSEQ11`"]
pub type ADSEQ11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQ11`"]
pub struct ADSEQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQ11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - ADSEQ8"]
    #[inline(always)]
    pub fn adseq8(&self) -> ADSEQ8_R {
        ADSEQ8_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - ADSEQ9"]
    #[inline(always)]
    pub fn adseq9(&self) -> ADSEQ9_R {
        ADSEQ9_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADSEQ10"]
    #[inline(always)]
    pub fn adseq10(&self) -> ADSEQ10_R {
        ADSEQ10_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADSEQ11"]
    #[inline(always)]
    pub fn adseq11(&self) -> ADSEQ11_R {
        ADSEQ11_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADSEQ8"]
    #[inline(always)]
    pub fn adseq8(&mut self) -> ADSEQ8_W {
        ADSEQ8_W { w: self }
    }
    #[doc = "Bits 8:12 - ADSEQ9"]
    #[inline(always)]
    pub fn adseq9(&mut self) -> ADSEQ9_W {
        ADSEQ9_W { w: self }
    }
    #[doc = "Bits 16:20 - ADSEQ10"]
    #[inline(always)]
    pub fn adseq10(&mut self) -> ADSEQ10_W {
        ADSEQ10_W { w: self }
    }
    #[doc = "Bits 24:28 - ADSEQ11"]
    #[inline(always)]
    pub fn adseq11(&mut self) -> ADSEQ11_W {
        ADSEQ11_W { w: self }
    }
}

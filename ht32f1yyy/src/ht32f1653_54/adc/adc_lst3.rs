#[doc = "Reader of register ADC_LST3"]
pub type R = crate::R<u32, super::ADC_LST3>;
#[doc = "Writer for register ADC_LST3"]
pub type W = crate::W<u32, super::ADC_LST3>;
#[doc = "Register ADC_LST3 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_LST3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADSEQ12`"]
pub type ADSEQ12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQ12`"]
pub struct ADSEQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQ12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ADSEQ13`"]
pub type ADSEQ13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQ13`"]
pub struct ADSEQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQ13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADSEQ14`"]
pub type ADSEQ14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQ14`"]
pub struct ADSEQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQ14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADSEQ15`"]
pub type ADSEQ15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQ15`"]
pub struct ADSEQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQ15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - ADSEQ12"]
    #[inline(always)]
    pub fn adseq12(&self) -> ADSEQ12_R {
        ADSEQ12_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - ADSEQ13"]
    #[inline(always)]
    pub fn adseq13(&self) -> ADSEQ13_R {
        ADSEQ13_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADSEQ14"]
    #[inline(always)]
    pub fn adseq14(&self) -> ADSEQ14_R {
        ADSEQ14_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADSEQ15"]
    #[inline(always)]
    pub fn adseq15(&self) -> ADSEQ15_R {
        ADSEQ15_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADSEQ12"]
    #[inline(always)]
    pub fn adseq12(&mut self) -> ADSEQ12_W {
        ADSEQ12_W { w: self }
    }
    #[doc = "Bits 8:12 - ADSEQ13"]
    #[inline(always)]
    pub fn adseq13(&mut self) -> ADSEQ13_W {
        ADSEQ13_W { w: self }
    }
    #[doc = "Bits 16:20 - ADSEQ14"]
    #[inline(always)]
    pub fn adseq14(&mut self) -> ADSEQ14_W {
        ADSEQ14_W { w: self }
    }
    #[doc = "Bits 24:28 - ADSEQ15"]
    #[inline(always)]
    pub fn adseq15(&mut self) -> ADSEQ15_W {
        ADSEQ15_W { w: self }
    }
}

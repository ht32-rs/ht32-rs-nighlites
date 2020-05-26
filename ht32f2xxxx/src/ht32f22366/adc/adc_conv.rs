#[doc = "Reader of register ADC_CONV"]
pub type R = crate::R<u32, super::ADC_CONV>;
#[doc = "Writer for register ADC_CONV"]
pub type W = crate::W<u32, super::ADC_CONV>;
#[doc = "Register ADC_CONV `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_CONV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADMODE`"]
pub type ADMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADMODE`"]
pub struct ADMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `ADSEQL`"]
pub type ADSEQL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSEQL`"]
pub struct ADSEQL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEQL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADSUBL`"]
pub type ADSUBL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADSUBL`"]
pub struct ADSUBL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSUBL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ADMODE"]
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - ADSEQL"]
    #[inline(always)]
    pub fn adseql(&self) -> ADSEQL_R {
        ADSEQL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADSUBL"]
    #[inline(always)]
    pub fn adsubl(&self) -> ADSUBL_R {
        ADSUBL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADMODE"]
    #[inline(always)]
    pub fn admode(&mut self) -> ADMODE_W {
        ADMODE_W { w: self }
    }
    #[doc = "Bits 8:11 - ADSEQL"]
    #[inline(always)]
    pub fn adseql(&mut self) -> ADSEQL_W {
        ADSEQL_W { w: self }
    }
    #[doc = "Bits 16:19 - ADSUBL"]
    #[inline(always)]
    pub fn adsubl(&mut self) -> ADSUBL_W {
        ADSUBL_W { w: self }
    }
}

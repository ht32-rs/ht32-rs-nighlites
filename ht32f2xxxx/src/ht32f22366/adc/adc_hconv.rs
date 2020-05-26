#[doc = "Reader of register ADC_HCONV"]
pub type R = crate::R<u32, super::ADC_HCONV>;
#[doc = "Writer for register ADC_HCONV"]
pub type W = crate::W<u32, super::ADC_HCONV>;
#[doc = "Register ADC_HCONV `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_HCONV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADHMODE`"]
pub type ADHMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADHMODE`"]
pub struct ADHMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `ADHSEQL`"]
pub type ADHSEQL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADHSEQL`"]
pub struct ADHSEQL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHSEQL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADHSUBL`"]
pub type ADHSUBL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADHSUBL`"]
pub struct ADHSUBL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHSUBL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ADHMODE"]
    #[inline(always)]
    pub fn adhmode(&self) -> ADHMODE_R {
        ADHMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - ADHSEQL"]
    #[inline(always)]
    pub fn adhseql(&self) -> ADHSEQL_R {
        ADHSEQL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - ADHSUBL"]
    #[inline(always)]
    pub fn adhsubl(&self) -> ADHSUBL_R {
        ADHSUBL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADHMODE"]
    #[inline(always)]
    pub fn adhmode(&mut self) -> ADHMODE_W {
        ADHMODE_W { w: self }
    }
    #[doc = "Bits 8:9 - ADHSEQL"]
    #[inline(always)]
    pub fn adhseql(&mut self) -> ADHSEQL_W {
        ADHSEQL_W { w: self }
    }
    #[doc = "Bits 16:17 - ADHSUBL"]
    #[inline(always)]
    pub fn adhsubl(&mut self) -> ADHSUBL_W {
        ADHSUBL_W { w: self }
    }
}

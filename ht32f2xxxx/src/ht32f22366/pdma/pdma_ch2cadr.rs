#[doc = "Reader of register PDMA_CH2CADR"]
pub type R = crate::R<u32, super::PDMA_CH2CADR>;
#[doc = "Writer for register PDMA_CH2CADR"]
pub type W = crate::W<u32, super::PDMA_CH2CADR>;
#[doc = "Register PDMA_CH2CADR `reset()`'s with value 0"]
impl crate::ResetValue for super::PDMA_CH2CADR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CDADR`"]
pub type CDADR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CDADR`"]
pub struct CDADR_W<'a> {
    w: &'a mut W,
}
impl<'a> CDADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `CSADR`"]
pub type CSADR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CSADR`"]
pub struct CSADR_W<'a> {
    w: &'a mut W,
}
impl<'a> CSADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CDADR"]
    #[inline(always)]
    pub fn cdadr(&self) -> CDADR_R {
        CDADR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CSADR"]
    #[inline(always)]
    pub fn csadr(&self) -> CSADR_R {
        CSADR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CDADR"]
    #[inline(always)]
    pub fn cdadr(&mut self) -> CDADR_W {
        CDADR_W { w: self }
    }
    #[doc = "Bits 16:31 - CSADR"]
    #[inline(always)]
    pub fn csadr(&mut self) -> CSADR_W {
        CSADR_W { w: self }
    }
}

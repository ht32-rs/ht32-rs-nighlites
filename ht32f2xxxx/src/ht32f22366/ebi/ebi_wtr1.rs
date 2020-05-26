#[doc = "Reader of register EBI_WTR1"]
pub type R = crate::R<u32, super::EBI_WTR1>;
#[doc = "Writer for register EBI_WTR1"]
pub type W = crate::W<u32, super::EBI_WTR1>;
#[doc = "Register EBI_WTR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EBI_WTR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRSETUP`"]
pub type WRSETUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRSETUP`"]
pub struct WRSETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> WRSETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `WRSTRB`"]
pub type WRSTRB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRSTRB`"]
pub struct WRSTRB_W<'a> {
    w: &'a mut W,
}
impl<'a> WRSTRB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `WRHOLD`"]
pub type WRHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRHOLD`"]
pub struct WRHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> WRHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - WRSETUP"]
    #[inline(always)]
    pub fn wrsetup(&self) -> WRSETUP_R {
        WRSETUP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - WRSTRB"]
    #[inline(always)]
    pub fn wrstrb(&self) -> WRSTRB_R {
        WRSTRB_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - WRHOLD"]
    #[inline(always)]
    pub fn wrhold(&self) -> WRHOLD_R {
        WRHOLD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - WRSETUP"]
    #[inline(always)]
    pub fn wrsetup(&mut self) -> WRSETUP_W {
        WRSETUP_W { w: self }
    }
    #[doc = "Bits 8:13 - WRSTRB"]
    #[inline(always)]
    pub fn wrstrb(&mut self) -> WRSTRB_W {
        WRSTRB_W { w: self }
    }
    #[doc = "Bits 16:19 - WRHOLD"]
    #[inline(always)]
    pub fn wrhold(&mut self) -> WRHOLD_W {
        WRHOLD_W { w: self }
    }
}

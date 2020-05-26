#[doc = "Reader of register CSIF_WCR1"]
pub type R = crate::R<u32, super::CSIF_WCR1>;
#[doc = "Writer for register CSIF_WCR1"]
pub type W = crate::W<u32, super::CSIF_WCR1>;
#[doc = "Register CSIF_WCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSIF_WCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WIN_WID`"]
pub type WIN_WID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WIN_WID`"]
pub struct WIN_WID_W<'a> {
    w: &'a mut W,
}
impl<'a> WIN_WID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `WIN_HGH`"]
pub type WIN_HGH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WIN_HGH`"]
pub struct WIN_HGH_W<'a> {
    w: &'a mut W,
}
impl<'a> WIN_HGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - WIN_WID"]
    #[inline(always)]
    pub fn win_wid(&self) -> WIN_WID_R {
        WIN_WID_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - WIN_HGH"]
    #[inline(always)]
    pub fn win_hgh(&self) -> WIN_HGH_R {
        WIN_HGH_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - WIN_WID"]
    #[inline(always)]
    pub fn win_wid(&mut self) -> WIN_WID_W {
        WIN_WID_W { w: self }
    }
    #[doc = "Bits 16:26 - WIN_HGH"]
    #[inline(always)]
    pub fn win_hgh(&mut self) -> WIN_HGH_W {
        WIN_HGH_W { w: self }
    }
}

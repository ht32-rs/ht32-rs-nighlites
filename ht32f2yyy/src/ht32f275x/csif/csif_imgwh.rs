#[doc = "Reader of register CSIF_IMGWH"]
pub type R = crate::R<u32, super::CSIF_IMGWH>;
#[doc = "Writer for register CSIF_IMGWH"]
pub type W = crate::W<u32, super::CSIF_IMGWH>;
#[doc = "Register CSIF_IMGWH `reset()`'s with value 0"]
impl crate::ResetValue for super::CSIF_IMGWH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IMG_WID`"]
pub type IMG_WID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IMG_WID`"]
pub struct IMG_WID_W<'a> {
    w: &'a mut W,
}
impl<'a> IMG_WID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `IMG_HGH`"]
pub type IMG_HGH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IMG_HGH`"]
pub struct IMG_HGH_W<'a> {
    w: &'a mut W,
}
impl<'a> IMG_HGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - IMG_WID"]
    #[inline(always)]
    pub fn img_wid(&self) -> IMG_WID_R {
        IMG_WID_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - IMG_HGH"]
    #[inline(always)]
    pub fn img_hgh(&self) -> IMG_HGH_R {
        IMG_HGH_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - IMG_WID"]
    #[inline(always)]
    pub fn img_wid(&mut self) -> IMG_WID_W {
        IMG_WID_W { w: self }
    }
    #[doc = "Bits 16:26 - IMG_HGH"]
    #[inline(always)]
    pub fn img_hgh(&mut self) -> IMG_HGH_W {
        IMG_HGH_W { w: self }
    }
}

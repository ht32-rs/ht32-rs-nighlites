#[doc = "Reader of register CSIF_WCR0"]
pub type R = crate::R<u32, super::CSIF_WCR0>;
#[doc = "Writer for register CSIF_WCR0"]
pub type W = crate::W<u32, super::CSIF_WCR0>;
#[doc = "Register CSIF_WCR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSIF_WCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WIN_HSTR`"]
pub type WIN_HSTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WIN_HSTR`"]
pub struct WIN_HSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> WIN_HSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `WIN_VSTR`"]
pub type WIN_VSTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WIN_VSTR`"]
pub struct WIN_VSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> WIN_VSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WIN_EN`"]
pub type WIN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WIN_EN`"]
pub struct WIN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIN_EN_W<'a> {
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
    #[doc = "Bits 0:10 - WIN_HSTR"]
    #[inline(always)]
    pub fn win_hstr(&self) -> WIN_HSTR_R {
        WIN_HSTR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - WIN_VSTR"]
    #[inline(always)]
    pub fn win_vstr(&self) -> WIN_VSTR_R {
        WIN_VSTR_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - WIN_EN"]
    #[inline(always)]
    pub fn win_en(&self) -> WIN_EN_R {
        WIN_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - WIN_HSTR"]
    #[inline(always)]
    pub fn win_hstr(&mut self) -> WIN_HSTR_W {
        WIN_HSTR_W { w: self }
    }
    #[doc = "Bits 16:26 - WIN_VSTR"]
    #[inline(always)]
    pub fn win_vstr(&mut self) -> WIN_VSTR_W {
        WIN_VSTR_W { w: self }
    }
    #[doc = "Bit 31 - WIN_EN"]
    #[inline(always)]
    pub fn win_en(&mut self) -> WIN_EN_W {
        WIN_EN_W { w: self }
    }
}

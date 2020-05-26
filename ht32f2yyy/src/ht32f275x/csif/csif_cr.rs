#[doc = "Reader of register CSIF_CR"]
pub type R = crate::R<u32, super::CSIF_CR>;
#[doc = "Writer for register CSIF_CR"]
pub type W = crate::W<u32, super::CSIF_CR>;
#[doc = "Register CSIF_CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSIF_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VSYNCTYP`"]
pub type VSYNCTYP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VSYNCTYP`"]
pub struct VSYNCTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNCTYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `HSYNCTYP`"]
pub type HSYNCTYP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSYNCTYP`"]
pub struct HSYNCTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> HSYNCTYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CLKEDGE`"]
pub type CLKEDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKEDGE`"]
pub struct CLKEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `IMG_FMT`"]
pub type IMG_FMT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMG_FMT`"]
pub struct IMG_FMT_W<'a> {
    w: &'a mut W,
}
impl<'a> IMG_FMT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `VSYNCPOL`"]
pub type VSYNCPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VSYNCPOL`"]
pub struct VSYNCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNCPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `HSYNCPOL`"]
pub type HSYNCPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSYNCPOL`"]
pub struct HSYNCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSYNCPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `IMG_SLD`"]
pub type IMG_SLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IMG_SLD`"]
pub struct IMG_SLD_W<'a> {
    w: &'a mut W,
}
impl<'a> IMG_SLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `IMG_SFD`"]
pub type IMG_SFD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IMG_SFD`"]
pub struct IMG_SFD_W<'a> {
    w: &'a mut W,
}
impl<'a> IMG_SFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - VSYNCTYP"]
    #[inline(always)]
    pub fn vsynctyp(&self) -> VSYNCTYP_R {
        VSYNCTYP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSYNCTYP"]
    #[inline(always)]
    pub fn hsynctyp(&self) -> HSYNCTYP_R {
        HSYNCTYP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CLKEDGE"]
    #[inline(always)]
    pub fn clkedge(&self) -> CLKEDGE_R {
        CLKEDGE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IMG_FMT"]
    #[inline(always)]
    pub fn img_fmt(&self) -> IMG_FMT_R {
        IMG_FMT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - VSYNCPOL"]
    #[inline(always)]
    pub fn vsyncpol(&self) -> VSYNCPOL_R {
        VSYNCPOL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - HSYNCPOL"]
    #[inline(always)]
    pub fn hsyncpol(&self) -> HSYNCPOL_R {
        HSYNCPOL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - IMG_SLD"]
    #[inline(always)]
    pub fn img_sld(&self) -> IMG_SLD_R {
        IMG_SLD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - IMG_SFD"]
    #[inline(always)]
    pub fn img_sfd(&self) -> IMG_SFD_R {
        IMG_SFD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - VSYNCTYP"]
    #[inline(always)]
    pub fn vsynctyp(&mut self) -> VSYNCTYP_W {
        VSYNCTYP_W { w: self }
    }
    #[doc = "Bit 2 - HSYNCTYP"]
    #[inline(always)]
    pub fn hsynctyp(&mut self) -> HSYNCTYP_W {
        HSYNCTYP_W { w: self }
    }
    #[doc = "Bit 3 - CLKEDGE"]
    #[inline(always)]
    pub fn clkedge(&mut self) -> CLKEDGE_W {
        CLKEDGE_W { w: self }
    }
    #[doc = "Bit 4 - IMG_FMT"]
    #[inline(always)]
    pub fn img_fmt(&mut self) -> IMG_FMT_W {
        IMG_FMT_W { w: self }
    }
    #[doc = "Bit 6 - VSYNCPOL"]
    #[inline(always)]
    pub fn vsyncpol(&mut self) -> VSYNCPOL_W {
        VSYNCPOL_W { w: self }
    }
    #[doc = "Bit 7 - HSYNCPOL"]
    #[inline(always)]
    pub fn hsyncpol(&mut self) -> HSYNCPOL_W {
        HSYNCPOL_W { w: self }
    }
    #[doc = "Bits 8:15 - IMG_SLD"]
    #[inline(always)]
    pub fn img_sld(&mut self) -> IMG_SLD_W {
        IMG_SLD_W { w: self }
    }
    #[doc = "Bits 16:23 - IMG_SFD"]
    #[inline(always)]
    pub fn img_sfd(&mut self) -> IMG_SFD_W {
        IMG_SFD_W { w: self }
    }
}

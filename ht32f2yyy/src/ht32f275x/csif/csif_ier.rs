#[doc = "Reader of register CSIF_IER"]
pub type R = crate::R<u32, super::CSIF_IER>;
#[doc = "Writer for register CSIF_IER"]
pub type W = crate::W<u32, super::CSIF_IER>;
#[doc = "Register CSIF_IER `reset()`'s with value 0"]
impl crate::ResetValue for super::CSIF_IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOFFLGE`"]
pub type SOFFLGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFFLGE`"]
pub struct SOFFLGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFFLGE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `EOFFLGE`"]
pub type EOFFLGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOFFLGE`"]
pub struct EOFFLGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOFFLGE_W<'a> {
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
#[doc = "Reader of field `CAPSTAE`"]
pub type CAPSTAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPSTAE`"]
pub struct CAPSTAE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPSTAE_W<'a> {
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
#[doc = "Reader of field `CAPSTSE`"]
pub type CAPSTSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAPSTSE`"]
pub struct CAPSTSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPSTSE_W<'a> {
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
#[doc = "Reader of field `BADFRME`"]
pub type BADFRME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BADFRME`"]
pub struct BADFRME_W<'a> {
    w: &'a mut W,
}
impl<'a> BADFRME_W<'a> {
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
#[doc = "Reader of field `FIFOOVRE`"]
pub type FIFOOVRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFOOVRE`"]
pub struct FIFOOVRE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOOVRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `FIFOEMPE`"]
pub type FIFOEMPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFOEMPE`"]
pub struct FIFOEMPE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOEMPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `FIFOFULE`"]
pub type FIFOFULE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFOFULE`"]
pub struct FIFOFULE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOFULE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SOFFLGE"]
    #[inline(always)]
    pub fn sofflge(&self) -> SOFFLGE_R {
        SOFFLGE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EOFFLGE"]
    #[inline(always)]
    pub fn eofflge(&self) -> EOFFLGE_R {
        EOFFLGE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CAPSTAE"]
    #[inline(always)]
    pub fn capstae(&self) -> CAPSTAE_R {
        CAPSTAE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CAPSTSE"]
    #[inline(always)]
    pub fn capstse(&self) -> CAPSTSE_R {
        CAPSTSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BADFRME"]
    #[inline(always)]
    pub fn badfrme(&self) -> BADFRME_R {
        BADFRME_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FIFOOVRE"]
    #[inline(always)]
    pub fn fifoovre(&self) -> FIFOOVRE_R {
        FIFOOVRE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FIFOEMPE"]
    #[inline(always)]
    pub fn fifoempe(&self) -> FIFOEMPE_R {
        FIFOEMPE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FIFOFULE"]
    #[inline(always)]
    pub fn fifofule(&self) -> FIFOFULE_R {
        FIFOFULE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SOFFLGE"]
    #[inline(always)]
    pub fn sofflge(&mut self) -> SOFFLGE_W {
        SOFFLGE_W { w: self }
    }
    #[doc = "Bit 1 - EOFFLGE"]
    #[inline(always)]
    pub fn eofflge(&mut self) -> EOFFLGE_W {
        EOFFLGE_W { w: self }
    }
    #[doc = "Bit 2 - CAPSTAE"]
    #[inline(always)]
    pub fn capstae(&mut self) -> CAPSTAE_W {
        CAPSTAE_W { w: self }
    }
    #[doc = "Bit 3 - CAPSTSE"]
    #[inline(always)]
    pub fn capstse(&mut self) -> CAPSTSE_W {
        CAPSTSE_W { w: self }
    }
    #[doc = "Bit 4 - BADFRME"]
    #[inline(always)]
    pub fn badfrme(&mut self) -> BADFRME_W {
        BADFRME_W { w: self }
    }
    #[doc = "Bit 8 - FIFOOVRE"]
    #[inline(always)]
    pub fn fifoovre(&mut self) -> FIFOOVRE_W {
        FIFOOVRE_W { w: self }
    }
    #[doc = "Bit 9 - FIFOEMPE"]
    #[inline(always)]
    pub fn fifoempe(&mut self) -> FIFOEMPE_W {
        FIFOEMPE_W { w: self }
    }
    #[doc = "Bit 10 - FIFOFULE"]
    #[inline(always)]
    pub fn fifofule(&mut self) -> FIFOFULE_W {
        FIFOFULE_W { w: self }
    }
}

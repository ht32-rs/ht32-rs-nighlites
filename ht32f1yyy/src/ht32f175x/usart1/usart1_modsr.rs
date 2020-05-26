#[doc = "Reader of register USART1_MODSR"]
pub type R = crate::R<u32, super::USART1_MODSR>;
#[doc = "Writer for register USART1_MODSR"]
pub type W = crate::W<u32, super::USART1_MODSR>;
#[doc = "Register USART1_MODSR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART1_MODSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCTS`"]
pub type DCTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCTS`"]
pub struct DCTS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCTS_W<'a> {
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
#[doc = "Reader of field `DDSR`"]
pub type DDSR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDSR`"]
pub struct DDSR_W<'a> {
    w: &'a mut W,
}
impl<'a> DDSR_W<'a> {
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
#[doc = "Reader of field `DRI`"]
pub type DRI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRI`"]
pub struct DRI_W<'a> {
    w: &'a mut W,
}
impl<'a> DRI_W<'a> {
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
#[doc = "Reader of field `DDCD`"]
pub type DDCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDCD`"]
pub struct DDCD_W<'a> {
    w: &'a mut W,
}
impl<'a> DDCD_W<'a> {
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
#[doc = "Reader of field `CTSS`"]
pub type CTSS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSS`"]
pub struct CTSS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSS_W<'a> {
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
#[doc = "Reader of field `DSRS`"]
pub type DSRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSRS`"]
pub struct DSRS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RIS`"]
pub type RIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RIS`"]
pub struct RIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RIS_W<'a> {
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
#[doc = "Reader of field `DCDS`"]
pub type DCDS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDS`"]
pub struct DCDS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DCTS"]
    #[inline(always)]
    pub fn dcts(&self) -> DCTS_R {
        DCTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DDSR"]
    #[inline(always)]
    pub fn ddsr(&self) -> DDSR_R {
        DDSR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DRI"]
    #[inline(always)]
    pub fn dri(&self) -> DRI_R {
        DRI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DDCD"]
    #[inline(always)]
    pub fn ddcd(&self) -> DDCD_R {
        DDCD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CTSS"]
    #[inline(always)]
    pub fn ctss(&self) -> CTSS_R {
        CTSS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DSRS"]
    #[inline(always)]
    pub fn dsrs(&self) -> DSRS_R {
        DSRS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RIS"]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DCDS"]
    #[inline(always)]
    pub fn dcds(&self) -> DCDS_R {
        DCDS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCTS"]
    #[inline(always)]
    pub fn dcts(&mut self) -> DCTS_W {
        DCTS_W { w: self }
    }
    #[doc = "Bit 1 - DDSR"]
    #[inline(always)]
    pub fn ddsr(&mut self) -> DDSR_W {
        DDSR_W { w: self }
    }
    #[doc = "Bit 2 - DRI"]
    #[inline(always)]
    pub fn dri(&mut self) -> DRI_W {
        DRI_W { w: self }
    }
    #[doc = "Bit 3 - DDCD"]
    #[inline(always)]
    pub fn ddcd(&mut self) -> DDCD_W {
        DDCD_W { w: self }
    }
    #[doc = "Bit 4 - CTSS"]
    #[inline(always)]
    pub fn ctss(&mut self) -> CTSS_W {
        CTSS_W { w: self }
    }
    #[doc = "Bit 5 - DSRS"]
    #[inline(always)]
    pub fn dsrs(&mut self) -> DSRS_W {
        DSRS_W { w: self }
    }
    #[doc = "Bit 6 - RIS"]
    #[inline(always)]
    pub fn ris(&mut self) -> RIS_W {
        RIS_W { w: self }
    }
    #[doc = "Bit 7 - DCDS"]
    #[inline(always)]
    pub fn dcds(&mut self) -> DCDS_W {
        DCDS_W { w: self }
    }
}

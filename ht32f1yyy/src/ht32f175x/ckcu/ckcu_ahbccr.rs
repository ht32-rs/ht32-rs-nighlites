#[doc = "Reader of register CKCU_AHBCCR"]
pub type R = crate::R<u32, super::CKCU_AHBCCR>;
#[doc = "Writer for register CKCU_AHBCCR"]
pub type W = crate::W<u32, super::CKCU_AHBCCR>;
#[doc = "Register CKCU_AHBCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CKCU_AHBCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FMCEN`"]
pub type FMCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMCEN`"]
pub struct FMCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCEN_W<'a> {
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
#[doc = "Reader of field `SRAMEN`"]
pub type SRAMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAMEN`"]
pub struct SRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMEN_W<'a> {
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
#[doc = "Reader of field `PDMAEN`"]
pub type PDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDMAEN`"]
pub struct PDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMAEN_W<'a> {
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
#[doc = "Reader of field `BMEN`"]
pub type BMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BMEN`"]
pub struct BMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BMEN_W<'a> {
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
#[doc = "Reader of field `APB0EN`"]
pub type APB0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB0EN`"]
pub struct APB0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB0EN_W<'a> {
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
#[doc = "Reader of field `APB1EN`"]
pub type APB1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB1EN`"]
pub struct APB1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB1EN_W<'a> {
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
    #[doc = "Bit 0 - FMCEN"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - SRAMEN"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PDMAEN"]
    #[inline(always)]
    pub fn pdmaen(&self) -> PDMAEN_R {
        PDMAEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BMEN"]
    #[inline(always)]
    pub fn bmen(&self) -> BMEN_R {
        BMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - APB0EN"]
    #[inline(always)]
    pub fn apb0en(&self) -> APB0EN_R {
        APB0EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - APB1EN"]
    #[inline(always)]
    pub fn apb1en(&self) -> APB1EN_R {
        APB1EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FMCEN"]
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W {
        FMCEN_W { w: self }
    }
    #[doc = "Bit 2 - SRAMEN"]
    #[inline(always)]
    pub fn sramen(&mut self) -> SRAMEN_W {
        SRAMEN_W { w: self }
    }
    #[doc = "Bit 4 - PDMAEN"]
    #[inline(always)]
    pub fn pdmaen(&mut self) -> PDMAEN_W {
        PDMAEN_W { w: self }
    }
    #[doc = "Bit 5 - BMEN"]
    #[inline(always)]
    pub fn bmen(&mut self) -> BMEN_W {
        BMEN_W { w: self }
    }
    #[doc = "Bit 6 - APB0EN"]
    #[inline(always)]
    pub fn apb0en(&mut self) -> APB0EN_W {
        APB0EN_W { w: self }
    }
    #[doc = "Bit 7 - APB1EN"]
    #[inline(always)]
    pub fn apb1en(&mut self) -> APB1EN_W {
        APB1EN_W { w: self }
    }
}

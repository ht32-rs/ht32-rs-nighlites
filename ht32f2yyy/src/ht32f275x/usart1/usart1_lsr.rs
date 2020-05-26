#[doc = "Reader of register USART1_LSR"]
pub type R = crate::R<u32, super::USART1_LSR>;
#[doc = "Writer for register USART1_LSR"]
pub type W = crate::W<u32, super::USART1_LSR>;
#[doc = "Register USART1_LSR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART1_LSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFDR`"]
pub type RFDR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFDR`"]
pub struct RFDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RFDR_W<'a> {
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
#[doc = "Reader of field `OEI`"]
pub type OEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEI`"]
pub struct OEI_W<'a> {
    w: &'a mut W,
}
impl<'a> OEI_W<'a> {
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
#[doc = "Reader of field `PEI`"]
pub type PEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEI`"]
pub struct PEI_W<'a> {
    w: &'a mut W,
}
impl<'a> PEI_W<'a> {
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
#[doc = "Reader of field `FEI`"]
pub type FEI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEI`"]
pub struct FEI_W<'a> {
    w: &'a mut W,
}
impl<'a> FEI_W<'a> {
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
#[doc = "Reader of field `BII`"]
pub type BII_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BII`"]
pub struct BII_W<'a> {
    w: &'a mut W,
}
impl<'a> BII_W<'a> {
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
#[doc = "Reader of field `TXFEMPT`"]
pub type TXFEMPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFEMPT`"]
pub struct TXFEMPT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFEMPT_W<'a> {
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
#[doc = "Reader of field `TXEMPT`"]
pub type TXEMPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXEMPT`"]
pub struct TXEMPT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEMPT_W<'a> {
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
#[doc = "Reader of field `ERRRX`"]
pub type ERRRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRRX`"]
pub struct ERRRX_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRRX_W<'a> {
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
#[doc = "Reader of field `RSADDEF`"]
pub type RSADDEF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSADDEF`"]
pub struct RSADDEF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSADDEF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RFDR"]
    #[inline(always)]
    pub fn rfdr(&self) -> RFDR_R {
        RFDR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OEI"]
    #[inline(always)]
    pub fn oei(&self) -> OEI_R {
        OEI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PEI"]
    #[inline(always)]
    pub fn pei(&self) -> PEI_R {
        PEI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FEI"]
    #[inline(always)]
    pub fn fei(&self) -> FEI_R {
        FEI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BII"]
    #[inline(always)]
    pub fn bii(&self) -> BII_R {
        BII_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TXFEMPT"]
    #[inline(always)]
    pub fn txfempt(&self) -> TXFEMPT_R {
        TXFEMPT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TXEMPT"]
    #[inline(always)]
    pub fn txempt(&self) -> TXEMPT_R {
        TXEMPT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ERRRX"]
    #[inline(always)]
    pub fn errrx(&self) -> ERRRX_R {
        ERRRX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RSADDEF"]
    #[inline(always)]
    pub fn rsaddef(&self) -> RSADDEF_R {
        RSADDEF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RFDR"]
    #[inline(always)]
    pub fn rfdr(&mut self) -> RFDR_W {
        RFDR_W { w: self }
    }
    #[doc = "Bit 1 - OEI"]
    #[inline(always)]
    pub fn oei(&mut self) -> OEI_W {
        OEI_W { w: self }
    }
    #[doc = "Bit 2 - PEI"]
    #[inline(always)]
    pub fn pei(&mut self) -> PEI_W {
        PEI_W { w: self }
    }
    #[doc = "Bit 3 - FEI"]
    #[inline(always)]
    pub fn fei(&mut self) -> FEI_W {
        FEI_W { w: self }
    }
    #[doc = "Bit 4 - BII"]
    #[inline(always)]
    pub fn bii(&mut self) -> BII_W {
        BII_W { w: self }
    }
    #[doc = "Bit 5 - TXFEMPT"]
    #[inline(always)]
    pub fn txfempt(&mut self) -> TXFEMPT_W {
        TXFEMPT_W { w: self }
    }
    #[doc = "Bit 6 - TXEMPT"]
    #[inline(always)]
    pub fn txempt(&mut self) -> TXEMPT_W {
        TXEMPT_W { w: self }
    }
    #[doc = "Bit 7 - ERRRX"]
    #[inline(always)]
    pub fn errrx(&mut self) -> ERRRX_W {
        ERRRX_W { w: self }
    }
    #[doc = "Bit 8 - RSADDEF"]
    #[inline(always)]
    pub fn rsaddef(&mut self) -> RSADDEF_W {
        RSADDEF_W { w: self }
    }
}

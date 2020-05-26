#[doc = "Reader of register USART1_IER"]
pub type R = crate::R<u32, super::USART1_IER>;
#[doc = "Writer for register USART1_IER"]
pub type W = crate::W<u32, super::USART1_IER>;
#[doc = "Register USART1_IER `reset()`'s with value 0"]
impl crate::ResetValue for super::USART1_IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFTLI_RTOIE`"]
pub type RFTLI_RTOIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFTLI_RTOIE`"]
pub struct RFTLI_RTOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFTLI_RTOIE_W<'a> {
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
#[doc = "Reader of field `TFTLIE`"]
pub type TFTLIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFTLIE`"]
pub struct TFTLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFTLIE_W<'a> {
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
#[doc = "Reader of field `RLSIE`"]
pub type RLSIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RLSIE`"]
pub struct RLSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RLSIE_W<'a> {
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
#[doc = "Reader of field `MODSIE`"]
pub type MODSIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODSIE`"]
pub struct MODSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODSIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RFTLI_RTOIE"]
    #[inline(always)]
    pub fn rftli_rtoie(&self) -> RFTLI_RTOIE_R {
        RFTLI_RTOIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TFTLIE"]
    #[inline(always)]
    pub fn tftlie(&self) -> TFTLIE_R {
        TFTLIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RLSIE"]
    #[inline(always)]
    pub fn rlsie(&self) -> RLSIE_R {
        RLSIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MODSIE"]
    #[inline(always)]
    pub fn modsie(&self) -> MODSIE_R {
        MODSIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RFTLI_RTOIE"]
    #[inline(always)]
    pub fn rftli_rtoie(&mut self) -> RFTLI_RTOIE_W {
        RFTLI_RTOIE_W { w: self }
    }
    #[doc = "Bit 1 - TFTLIE"]
    #[inline(always)]
    pub fn tftlie(&mut self) -> TFTLIE_W {
        TFTLIE_W { w: self }
    }
    #[doc = "Bit 2 - RLSIE"]
    #[inline(always)]
    pub fn rlsie(&mut self) -> RLSIE_W {
        RLSIE_W { w: self }
    }
    #[doc = "Bit 3 - MODSIE"]
    #[inline(always)]
    pub fn modsie(&mut self) -> MODSIE_W {
        MODSIE_W { w: self }
    }
}

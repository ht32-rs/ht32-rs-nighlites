#[doc = "Reader of register USART0_MODCR"]
pub type R = crate::R<u32, super::USART0_MODCR>;
#[doc = "Writer for register USART0_MODCR"]
pub type W = crate::W<u32, super::USART0_MODCR>;
#[doc = "Register USART0_MODCR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART0_MODCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTR`"]
pub type DTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTR`"]
pub struct DTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTR_W<'a> {
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
#[doc = "Reader of field `RTS`"]
pub type RTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTS`"]
pub struct RTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_W<'a> {
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
#[doc = "Reader of field `HFCEN`"]
pub type HFCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFCEN`"]
pub struct HFCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HFCEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DTR"]
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTS"]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HFCEN"]
    #[inline(always)]
    pub fn hfcen(&self) -> HFCEN_R {
        HFCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTR"]
    #[inline(always)]
    pub fn dtr(&mut self) -> DTR_W {
        DTR_W { w: self }
    }
    #[doc = "Bit 1 - RTS"]
    #[inline(always)]
    pub fn rts(&mut self) -> RTS_W {
        RTS_W { w: self }
    }
    #[doc = "Bit 2 - HFCEN"]
    #[inline(always)]
    pub fn hfcen(&mut self) -> HFCEN_W {
        HFCEN_W { w: self }
    }
}

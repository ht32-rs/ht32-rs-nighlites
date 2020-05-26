#[doc = "Reader of register MCTM_CH1OCFR"]
pub type R = crate::R<u32, super::MCTM_CH1OCFR>;
#[doc = "Writer for register MCTM_CH1OCFR"]
pub type W = crate::W<u32, super::MCTM_CH1OCFR>;
#[doc = "Register MCTM_CH1OCFR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCTM_CH1OCFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH1OM`"]
pub type CH1OM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH1OM`"]
pub struct CH1OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `REF1CE`"]
pub type REF1CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REF1CE`"]
pub struct REF1CE_W<'a> {
    w: &'a mut W,
}
impl<'a> REF1CE_W<'a> {
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
#[doc = "Reader of field `CH1PRE`"]
pub type CH1PRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1PRE`"]
pub struct CH1PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1PRE_W<'a> {
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
#[doc = "Reader of field `CH1IMAE`"]
pub type CH1IMAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1IMAE`"]
pub struct CH1IMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1IMAE_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - CH1OM"]
    #[inline(always)]
    pub fn ch1om(&self) -> CH1OM_R {
        CH1OM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - REF1CE"]
    #[inline(always)]
    pub fn ref1ce(&self) -> REF1CE_R {
        REF1CE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CH1PRE"]
    #[inline(always)]
    pub fn ch1pre(&self) -> CH1PRE_R {
        CH1PRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CH1IMAE"]
    #[inline(always)]
    pub fn ch1imae(&self) -> CH1IMAE_R {
        CH1IMAE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH1OM"]
    #[inline(always)]
    pub fn ch1om(&mut self) -> CH1OM_W {
        CH1OM_W { w: self }
    }
    #[doc = "Bit 3 - REF1CE"]
    #[inline(always)]
    pub fn ref1ce(&mut self) -> REF1CE_W {
        REF1CE_W { w: self }
    }
    #[doc = "Bit 4 - CH1PRE"]
    #[inline(always)]
    pub fn ch1pre(&mut self) -> CH1PRE_W {
        CH1PRE_W { w: self }
    }
    #[doc = "Bit 5 - CH1IMAE"]
    #[inline(always)]
    pub fn ch1imae(&mut self) -> CH1IMAE_W {
        CH1IMAE_W { w: self }
    }
}

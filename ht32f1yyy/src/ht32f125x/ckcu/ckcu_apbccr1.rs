#[doc = "Reader of register CKCU_APBCCR1"]
pub type R = crate::R<u32, super::CKCU_APBCCR1>;
#[doc = "Writer for register CKCU_APBCCR1"]
pub type W = crate::W<u32, super::CKCU_APBCCR1>;
#[doc = "Register CKCU_APBCCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CKCU_APBCCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDTEN`"]
pub type WDTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTEN`"]
pub struct WDTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTEN_W<'a> {
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
#[doc = "Reader of field `RTCEN`"]
pub type RTCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCEN`"]
pub struct RTCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCEN_W<'a> {
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
#[doc = "Reader of field `GPTM0EN`"]
pub type GPTM0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPTM0EN`"]
pub struct GPTM0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTM0EN_W<'a> {
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
#[doc = "Reader of field `GPTM1EN`"]
pub type GPTM1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPTM1EN`"]
pub struct GPTM1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTM1EN_W<'a> {
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
#[doc = "Reader of field `OPA0EN`"]
pub type OPA0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA0EN`"]
pub struct OPA0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA0EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `OPA1EN`"]
pub type OPA1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPA1EN`"]
pub struct OPA1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ADCEN`"]
pub type ADCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCEN`"]
pub struct ADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - WDTEN"]
    #[inline(always)]
    pub fn wdten(&self) -> WDTEN_R {
        WDTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RTCEN"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM0EN"]
    #[inline(always)]
    pub fn gptm0en(&self) -> GPTM0EN_R {
        GPTM0EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM1EN"]
    #[inline(always)]
    pub fn gptm1en(&self) -> GPTM1EN_R {
        GPTM1EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 22 - OPA0EN"]
    #[inline(always)]
    pub fn opa0en(&self) -> OPA0EN_R {
        OPA0EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - OPA1EN"]
    #[inline(always)]
    pub fn opa1en(&self) -> OPA1EN_R {
        OPA1EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADCEN"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - WDTEN"]
    #[inline(always)]
    pub fn wdten(&mut self) -> WDTEN_W {
        WDTEN_W { w: self }
    }
    #[doc = "Bit 6 - RTCEN"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W {
        RTCEN_W { w: self }
    }
    #[doc = "Bit 8 - GPTM0EN"]
    #[inline(always)]
    pub fn gptm0en(&mut self) -> GPTM0EN_W {
        GPTM0EN_W { w: self }
    }
    #[doc = "Bit 9 - GPTM1EN"]
    #[inline(always)]
    pub fn gptm1en(&mut self) -> GPTM1EN_W {
        GPTM1EN_W { w: self }
    }
    #[doc = "Bit 22 - OPA0EN"]
    #[inline(always)]
    pub fn opa0en(&mut self) -> OPA0EN_W {
        OPA0EN_W { w: self }
    }
    #[doc = "Bit 23 - OPA1EN"]
    #[inline(always)]
    pub fn opa1en(&mut self) -> OPA1EN_W {
        OPA1EN_W { w: self }
    }
    #[doc = "Bit 24 - ADCEN"]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W { w: self }
    }
}

#[doc = "Reader of register ADC_TCR"]
pub type R = crate::R<u32, super::ADC_TCR>;
#[doc = "Writer for register ADC_TCR"]
pub type W = crate::W<u32, super::ADC_TCR>;
#[doc = "Register ADC_TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADSW`"]
pub type ADSW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADSW`"]
pub struct ADSW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSW_W<'a> {
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
#[doc = "Reader of field `ADEXTI`"]
pub type ADEXTI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADEXTI`"]
pub struct ADEXTI_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEXTI_W<'a> {
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
#[doc = "Reader of field `GPTM`"]
pub type GPTM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPTM`"]
pub struct GPTM_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTM_W<'a> {
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
    #[doc = "Bit 0 - ADSW"]
    #[inline(always)]
    pub fn adsw(&self) -> ADSW_R {
        ADSW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADEXTI"]
    #[inline(always)]
    pub fn adexti(&self) -> ADEXTI_R {
        ADEXTI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPTM"]
    #[inline(always)]
    pub fn gptm(&self) -> GPTM_R {
        GPTM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADSW"]
    #[inline(always)]
    pub fn adsw(&mut self) -> ADSW_W {
        ADSW_W { w: self }
    }
    #[doc = "Bit 1 - ADEXTI"]
    #[inline(always)]
    pub fn adexti(&mut self) -> ADEXTI_W {
        ADEXTI_W { w: self }
    }
    #[doc = "Bit 2 - GPTM"]
    #[inline(always)]
    pub fn gptm(&mut self) -> GPTM_W {
        GPTM_W { w: self }
    }
}

#[doc = "Reader of register ADC_HTCR"]
pub type R = crate::R<u32, super::ADC_HTCR>;
#[doc = "Writer for register ADC_HTCR"]
pub type W = crate::W<u32, super::ADC_HTCR>;
#[doc = "Register ADC_HTCR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_HTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADHSW`"]
pub type ADHSW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADHSW`"]
pub struct ADHSW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHSW_W<'a> {
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
#[doc = "Reader of field `ADHEXTI`"]
pub type ADHEXTI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADHEXTI`"]
pub struct ADHEXTI_W<'a> {
    w: &'a mut W,
}
impl<'a> ADHEXTI_W<'a> {
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
#[doc = "Reader of field `HTM`"]
pub type HTM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTM`"]
pub struct HTM_W<'a> {
    w: &'a mut W,
}
impl<'a> HTM_W<'a> {
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
#[doc = "Reader of field `HBFTM`"]
pub type HBFTM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HBFTM`"]
pub struct HBFTM_W<'a> {
    w: &'a mut W,
}
impl<'a> HBFTM_W<'a> {
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
    #[doc = "Bit 0 - ADHSW"]
    #[inline(always)]
    pub fn adhsw(&self) -> ADHSW_R {
        ADHSW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADHEXTI"]
    #[inline(always)]
    pub fn adhexti(&self) -> ADHEXTI_R {
        ADHEXTI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HTM"]
    #[inline(always)]
    pub fn htm(&self) -> HTM_R {
        HTM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HBFTM"]
    #[inline(always)]
    pub fn hbftm(&self) -> HBFTM_R {
        HBFTM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADHSW"]
    #[inline(always)]
    pub fn adhsw(&mut self) -> ADHSW_W {
        ADHSW_W { w: self }
    }
    #[doc = "Bit 1 - ADHEXTI"]
    #[inline(always)]
    pub fn adhexti(&mut self) -> ADHEXTI_W {
        ADHEXTI_W { w: self }
    }
    #[doc = "Bit 2 - HTM"]
    #[inline(always)]
    pub fn htm(&mut self) -> HTM_W {
        HTM_W { w: self }
    }
    #[doc = "Bit 3 - HBFTM"]
    #[inline(always)]
    pub fn hbftm(&mut self) -> HBFTM_W {
        HBFTM_W { w: self }
    }
}

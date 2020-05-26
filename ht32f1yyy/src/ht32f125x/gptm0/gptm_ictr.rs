#[doc = "Reader of register GPTM_ICTR"]
pub type R = crate::R<u32, super::GPTM_ICTR>;
#[doc = "Writer for register GPTM_ICTR"]
pub type W = crate::W<u32, super::GPTM_ICTR>;
#[doc = "Register GPTM_ICTR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTM_ICTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0CCIE`"]
pub type CH0CCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0CCIE`"]
pub struct CH0CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0CCIE_W<'a> {
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
#[doc = "Reader of field `CH1CCIE`"]
pub type CH1CCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1CCIE`"]
pub struct CH1CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1CCIE_W<'a> {
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
#[doc = "Reader of field `CH2CCIE`"]
pub type CH2CCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2CCIE`"]
pub struct CH2CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2CCIE_W<'a> {
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
#[doc = "Reader of field `CH3CCIE`"]
pub type CH3CCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3CCIE`"]
pub struct CH3CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3CCIE_W<'a> {
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
#[doc = "Reader of field `UEVIE`"]
pub type UEVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UEVIE`"]
pub struct UEVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UEVIE_W<'a> {
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
#[doc = "Reader of field `TEVIE`"]
pub type TEVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEVIE`"]
pub struct TEVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEVIE_W<'a> {
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
    #[doc = "Bit 0 - CH0CCIE"]
    #[inline(always)]
    pub fn ch0ccie(&self) -> CH0CCIE_R {
        CH0CCIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH1CCIE"]
    #[inline(always)]
    pub fn ch1ccie(&self) -> CH1CCIE_R {
        CH1CCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CH2CCIE"]
    #[inline(always)]
    pub fn ch2ccie(&self) -> CH2CCIE_R {
        CH2CCIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CH3CCIE"]
    #[inline(always)]
    pub fn ch3ccie(&self) -> CH3CCIE_R {
        CH3CCIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UEVIE"]
    #[inline(always)]
    pub fn uevie(&self) -> UEVIE_R {
        UEVIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TEVIE"]
    #[inline(always)]
    pub fn tevie(&self) -> TEVIE_R {
        TEVIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0CCIE"]
    #[inline(always)]
    pub fn ch0ccie(&mut self) -> CH0CCIE_W {
        CH0CCIE_W { w: self }
    }
    #[doc = "Bit 1 - CH1CCIE"]
    #[inline(always)]
    pub fn ch1ccie(&mut self) -> CH1CCIE_W {
        CH1CCIE_W { w: self }
    }
    #[doc = "Bit 2 - CH2CCIE"]
    #[inline(always)]
    pub fn ch2ccie(&mut self) -> CH2CCIE_W {
        CH2CCIE_W { w: self }
    }
    #[doc = "Bit 3 - CH3CCIE"]
    #[inline(always)]
    pub fn ch3ccie(&mut self) -> CH3CCIE_W {
        CH3CCIE_W { w: self }
    }
    #[doc = "Bit 8 - UEVIE"]
    #[inline(always)]
    pub fn uevie(&mut self) -> UEVIE_W {
        UEVIE_W { w: self }
    }
    #[doc = "Bit 10 - TEVIE"]
    #[inline(always)]
    pub fn tevie(&mut self) -> TEVIE_W {
        TEVIE_W { w: self }
    }
}

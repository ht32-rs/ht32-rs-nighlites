#[doc = "Reader of register USART_FCR"]
pub type R = crate::R<u32, super::USART_FCR>;
#[doc = "Writer for register USART_FCR"]
pub type W = crate::W<u32, super::USART_FCR>;
#[doc = "Register USART_FCR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART_FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FME`"]
pub type FME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FME`"]
pub struct FME_W<'a> {
    w: &'a mut W,
}
impl<'a> FME_W<'a> {
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
#[doc = "Reader of field `RFR`"]
pub type RFR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFR`"]
pub struct RFR_W<'a> {
    w: &'a mut W,
}
impl<'a> RFR_W<'a> {
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
#[doc = "Reader of field `TFR`"]
pub type TFR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFR`"]
pub struct TFR_W<'a> {
    w: &'a mut W,
}
impl<'a> TFR_W<'a> {
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
#[doc = "Reader of field `TFTL`"]
pub type TFTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TFTL`"]
pub struct TFTL_W<'a> {
    w: &'a mut W,
}
impl<'a> TFTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `RFTL`"]
pub type RFTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFTL`"]
pub struct RFTL_W<'a> {
    w: &'a mut W,
}
impl<'a> RFTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FME"]
    #[inline(always)]
    pub fn fme(&self) -> FME_R {
        FME_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RFR"]
    #[inline(always)]
    pub fn rfr(&self) -> RFR_R {
        RFR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TFR"]
    #[inline(always)]
    pub fn tfr(&self) -> TFR_R {
        TFR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - TFTL"]
    #[inline(always)]
    pub fn tftl(&self) -> TFTL_R {
        TFTL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - RFTL"]
    #[inline(always)]
    pub fn rftl(&self) -> RFTL_R {
        RFTL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FME"]
    #[inline(always)]
    pub fn fme(&mut self) -> FME_W {
        FME_W { w: self }
    }
    #[doc = "Bit 1 - RFR"]
    #[inline(always)]
    pub fn rfr(&mut self) -> RFR_W {
        RFR_W { w: self }
    }
    #[doc = "Bit 2 - TFR"]
    #[inline(always)]
    pub fn tfr(&mut self) -> TFR_W {
        TFR_W { w: self }
    }
    #[doc = "Bits 4:5 - TFTL"]
    #[inline(always)]
    pub fn tftl(&mut self) -> TFTL_W {
        TFTL_W { w: self }
    }
    #[doc = "Bits 6:7 - RFTL"]
    #[inline(always)]
    pub fn rftl(&mut self) -> RFTL_W {
        RFTL_W { w: self }
    }
}

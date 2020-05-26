#[doc = "Reader of register ADC_ICLR"]
pub type R = crate::R<u32, super::ADC_ICLR>;
#[doc = "Writer for register ADC_ICLR"]
pub type W = crate::W<u32, super::ADC_ICLR>;
#[doc = "Register ADC_ICLR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_ICLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADICLRS`"]
pub type ADICLRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADICLRS`"]
pub struct ADICLRS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADICLRS_W<'a> {
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
#[doc = "Reader of field `ADICLRG`"]
pub type ADICLRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADICLRG`"]
pub struct ADICLRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADICLRG_W<'a> {
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
#[doc = "Reader of field `ADICLRC`"]
pub type ADICLRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADICLRC`"]
pub struct ADICLRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADICLRC_W<'a> {
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
#[doc = "Reader of field `ADICLRHS`"]
pub type ADICLRHS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADICLRHS`"]
pub struct ADICLRHS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADICLRHS_W<'a> {
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
#[doc = "Reader of field `ADICLRHG`"]
pub type ADICLRHG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADICLRHG`"]
pub struct ADICLRHG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADICLRHG_W<'a> {
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
#[doc = "Reader of field `ADICLRHC`"]
pub type ADICLRHC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADICLRHC`"]
pub struct ADICLRHC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADICLRHC_W<'a> {
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
#[doc = "Reader of field `ADICLRL`"]
pub type ADICLRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADICLRL`"]
pub struct ADICLRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADICLRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADICLRU`"]
pub type ADICLRU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADICLRU`"]
pub struct ADICLRU_W<'a> {
    w: &'a mut W,
}
impl<'a> ADICLRU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `ADICLRO`"]
pub type ADICLRO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADICLRO`"]
pub struct ADICLRO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADICLRO_W<'a> {
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
#[doc = "Reader of field `ADICLRHO`"]
pub type ADICLRHO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADICLRHO`"]
pub struct ADICLRHO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADICLRHO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ADICLRS"]
    #[inline(always)]
    pub fn adiclrs(&self) -> ADICLRS_R {
        ADICLRS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADICLRG"]
    #[inline(always)]
    pub fn adiclrg(&self) -> ADICLRG_R {
        ADICLRG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADICLRC"]
    #[inline(always)]
    pub fn adiclrc(&self) -> ADICLRC_R {
        ADICLRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADICLRHS"]
    #[inline(always)]
    pub fn adiclrhs(&self) -> ADICLRHS_R {
        ADICLRHS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADICLRHG"]
    #[inline(always)]
    pub fn adiclrhg(&self) -> ADICLRHG_R {
        ADICLRHG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADICLRHC"]
    #[inline(always)]
    pub fn adiclrhc(&self) -> ADICLRHC_R {
        ADICLRHC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADICLRL"]
    #[inline(always)]
    pub fn adiclrl(&self) -> ADICLRL_R {
        ADICLRL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADICLRU"]
    #[inline(always)]
    pub fn adiclru(&self) -> ADICLRU_R {
        ADICLRU_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADICLRO"]
    #[inline(always)]
    pub fn adiclro(&self) -> ADICLRO_R {
        ADICLRO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ADICLRHO"]
    #[inline(always)]
    pub fn adiclrho(&self) -> ADICLRHO_R {
        ADICLRHO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADICLRS"]
    #[inline(always)]
    pub fn adiclrs(&mut self) -> ADICLRS_W {
        ADICLRS_W { w: self }
    }
    #[doc = "Bit 1 - ADICLRG"]
    #[inline(always)]
    pub fn adiclrg(&mut self) -> ADICLRG_W {
        ADICLRG_W { w: self }
    }
    #[doc = "Bit 2 - ADICLRC"]
    #[inline(always)]
    pub fn adiclrc(&mut self) -> ADICLRC_W {
        ADICLRC_W { w: self }
    }
    #[doc = "Bit 8 - ADICLRHS"]
    #[inline(always)]
    pub fn adiclrhs(&mut self) -> ADICLRHS_W {
        ADICLRHS_W { w: self }
    }
    #[doc = "Bit 9 - ADICLRHG"]
    #[inline(always)]
    pub fn adiclrhg(&mut self) -> ADICLRHG_W {
        ADICLRHG_W { w: self }
    }
    #[doc = "Bit 10 - ADICLRHC"]
    #[inline(always)]
    pub fn adiclrhc(&mut self) -> ADICLRHC_W {
        ADICLRHC_W { w: self }
    }
    #[doc = "Bit 16 - ADICLRL"]
    #[inline(always)]
    pub fn adiclrl(&mut self) -> ADICLRL_W {
        ADICLRL_W { w: self }
    }
    #[doc = "Bit 17 - ADICLRU"]
    #[inline(always)]
    pub fn adiclru(&mut self) -> ADICLRU_W {
        ADICLRU_W { w: self }
    }
    #[doc = "Bit 24 - ADICLRO"]
    #[inline(always)]
    pub fn adiclro(&mut self) -> ADICLRO_W {
        ADICLRO_W { w: self }
    }
    #[doc = "Bit 25 - ADICLRHO"]
    #[inline(always)]
    pub fn adiclrho(&mut self) -> ADICLRHO_W {
        ADICLRHO_W { w: self }
    }
}

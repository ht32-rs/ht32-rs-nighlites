#[doc = "Reader of register OPACR0"]
pub type R = crate::R<u32, super::OPACR0>;
#[doc = "Writer for register OPACR0"]
pub type W = crate::W<u32, super::OPACR0>;
#[doc = "Register OPACR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::OPACR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `OPC0MS`"]
pub type OPC0MS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPC0MS`"]
pub struct OPC0MS_W<'a> {
    w: &'a mut W,
}
impl<'a> OPC0MS_W<'a> {
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
#[doc = "Reader of field `A0OFM`"]
pub type A0OFM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A0OFM`"]
pub struct A0OFM_W<'a> {
    w: &'a mut W,
}
impl<'a> A0OFM_W<'a> {
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
#[doc = "Reader of field `A0RS`"]
pub type A0RS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A0RS`"]
pub struct A0RS_W<'a> {
    w: &'a mut W,
}
impl<'a> A0RS_W<'a> {
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
#[doc = "Reader of field `CMP0S`"]
pub type CMP0S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP0S`"]
pub struct CMP0S_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0S_W<'a> {
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
    #[doc = "Bit 0 - OPA0EN"]
    #[inline(always)]
    pub fn opa0en(&self) -> OPA0EN_R {
        OPA0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OPC0MS"]
    #[inline(always)]
    pub fn opc0ms(&self) -> OPC0MS_R {
        OPC0MS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - A0OFM"]
    #[inline(always)]
    pub fn a0ofm(&self) -> A0OFM_R {
        A0OFM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - A0RS"]
    #[inline(always)]
    pub fn a0rs(&self) -> A0RS_R {
        A0RS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CMP0S"]
    #[inline(always)]
    pub fn cmp0s(&self) -> CMP0S_R {
        CMP0S_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPA0EN"]
    #[inline(always)]
    pub fn opa0en(&mut self) -> OPA0EN_W {
        OPA0EN_W { w: self }
    }
    #[doc = "Bit 1 - OPC0MS"]
    #[inline(always)]
    pub fn opc0ms(&mut self) -> OPC0MS_W {
        OPC0MS_W { w: self }
    }
    #[doc = "Bit 2 - A0OFM"]
    #[inline(always)]
    pub fn a0ofm(&mut self) -> A0OFM_W {
        A0OFM_W { w: self }
    }
    #[doc = "Bit 3 - A0RS"]
    #[inline(always)]
    pub fn a0rs(&mut self) -> A0RS_W {
        A0RS_W { w: self }
    }
    #[doc = "Bit 8 - CMP0S"]
    #[inline(always)]
    pub fn cmp0s(&mut self) -> CMP0S_W {
        CMP0S_W { w: self }
    }
}

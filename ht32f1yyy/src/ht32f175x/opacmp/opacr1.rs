#[doc = "Reader of register OPACR1"]
pub type R = crate::R<u32, super::OPACR1>;
#[doc = "Writer for register OPACR1"]
pub type W = crate::W<u32, super::OPACR1>;
#[doc = "Register OPACR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::OPACR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `OPC1MS`"]
pub type OPC1MS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OPC1MS`"]
pub struct OPC1MS_W<'a> {
    w: &'a mut W,
}
impl<'a> OPC1MS_W<'a> {
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
#[doc = "Reader of field `A1OFM`"]
pub type A1OFM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A1OFM`"]
pub struct A1OFM_W<'a> {
    w: &'a mut W,
}
impl<'a> A1OFM_W<'a> {
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
#[doc = "Reader of field `A1RS`"]
pub type A1RS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `A1RS`"]
pub struct A1RS_W<'a> {
    w: &'a mut W,
}
impl<'a> A1RS_W<'a> {
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
#[doc = "Reader of field `CMP1S`"]
pub type CMP1S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP1S`"]
pub struct CMP1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1S_W<'a> {
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
    #[doc = "Bit 0 - OPA1EN"]
    #[inline(always)]
    pub fn opa1en(&self) -> OPA1EN_R {
        OPA1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OPC1MS"]
    #[inline(always)]
    pub fn opc1ms(&self) -> OPC1MS_R {
        OPC1MS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - A1OFM"]
    #[inline(always)]
    pub fn a1ofm(&self) -> A1OFM_R {
        A1OFM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - A1RS"]
    #[inline(always)]
    pub fn a1rs(&self) -> A1RS_R {
        A1RS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CMP1S"]
    #[inline(always)]
    pub fn cmp1s(&self) -> CMP1S_R {
        CMP1S_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPA1EN"]
    #[inline(always)]
    pub fn opa1en(&mut self) -> OPA1EN_W {
        OPA1EN_W { w: self }
    }
    #[doc = "Bit 1 - OPC1MS"]
    #[inline(always)]
    pub fn opc1ms(&mut self) -> OPC1MS_W {
        OPC1MS_W { w: self }
    }
    #[doc = "Bit 2 - A1OFM"]
    #[inline(always)]
    pub fn a1ofm(&mut self) -> A1OFM_W {
        A1OFM_W { w: self }
    }
    #[doc = "Bit 3 - A1RS"]
    #[inline(always)]
    pub fn a1rs(&mut self) -> A1RS_W {
        A1RS_W { w: self }
    }
    #[doc = "Bit 8 - CMP1S"]
    #[inline(always)]
    pub fn cmp1s(&mut self) -> CMP1S_W {
        CMP1S_W { w: self }
    }
}

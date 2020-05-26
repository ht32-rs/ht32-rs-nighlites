#[doc = "Reader of register UFSR"]
pub type R = crate::R<u16, super::UFSR>;
#[doc = "Writer for register UFSR"]
pub type W = crate::W<u16, super::UFSR>;
#[doc = "Register UFSR `reset()`'s with value 0"]
impl crate::ResetValue for super::UFSR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UNDEFINSTR`"]
pub type UNDEFINSTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNDEFINSTR`"]
pub struct UNDEFINSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDEFINSTR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `INVSTATE`"]
pub type INVSTATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVSTATE`"]
pub struct INVSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> INVSTATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `INVPC`"]
pub type INVPC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVPC`"]
pub struct INVPC_W<'a> {
    w: &'a mut W,
}
impl<'a> INVPC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `NOCP`"]
pub type NOCP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOCP`"]
pub struct NOCP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOCP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `UNALIGNED`"]
pub type UNALIGNED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNALIGNED`"]
pub struct UNALIGNED_W<'a> {
    w: &'a mut W,
}
impl<'a> UNALIGNED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DIVBYZERO`"]
pub type DIVBYZERO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIVBYZERO`"]
pub struct DIVBYZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVBYZERO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - UNDEFINSTR"]
    #[inline(always)]
    pub fn undefinstr(&self) -> UNDEFINSTR_R {
        UNDEFINSTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - INVSTATE"]
    #[inline(always)]
    pub fn invstate(&self) -> INVSTATE_R {
        INVSTATE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - INVPC"]
    #[inline(always)]
    pub fn invpc(&self) -> INVPC_R {
        INVPC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NOCP"]
    #[inline(always)]
    pub fn nocp(&self) -> NOCP_R {
        NOCP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UNALIGNED"]
    #[inline(always)]
    pub fn unaligned(&self) -> UNALIGNED_R {
        UNALIGNED_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DIVBYZERO"]
    #[inline(always)]
    pub fn divbyzero(&self) -> DIVBYZERO_R {
        DIVBYZERO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UNDEFINSTR"]
    #[inline(always)]
    pub fn undefinstr(&mut self) -> UNDEFINSTR_W {
        UNDEFINSTR_W { w: self }
    }
    #[doc = "Bit 1 - INVSTATE"]
    #[inline(always)]
    pub fn invstate(&mut self) -> INVSTATE_W {
        INVSTATE_W { w: self }
    }
    #[doc = "Bit 2 - INVPC"]
    #[inline(always)]
    pub fn invpc(&mut self) -> INVPC_W {
        INVPC_W { w: self }
    }
    #[doc = "Bit 3 - NOCP"]
    #[inline(always)]
    pub fn nocp(&mut self) -> NOCP_W {
        NOCP_W { w: self }
    }
    #[doc = "Bit 8 - UNALIGNED"]
    #[inline(always)]
    pub fn unaligned(&mut self) -> UNALIGNED_W {
        UNALIGNED_W { w: self }
    }
    #[doc = "Bit 9 - DIVBYZERO"]
    #[inline(always)]
    pub fn divbyzero(&mut self) -> DIVBYZERO_W {
        DIVBYZERO_W { w: self }
    }
}

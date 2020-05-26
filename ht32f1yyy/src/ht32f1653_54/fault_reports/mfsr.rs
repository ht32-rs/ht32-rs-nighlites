#[doc = "Reader of register MFSR"]
pub type R = crate::R<u8, super::MFSR>;
#[doc = "Writer for register MFSR"]
pub type W = crate::W<u8, super::MFSR>;
#[doc = "Register MFSR `reset()`'s with value 0"]
impl crate::ResetValue for super::MFSR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IACCVIOL`"]
pub type IACCVIOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IACCVIOL`"]
pub struct IACCVIOL_W<'a> {
    w: &'a mut W,
}
impl<'a> IACCVIOL_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `DACCVIOL`"]
pub type DACCVIOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DACCVIOL`"]
pub struct DACCVIOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DACCVIOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `MUNSTKERR`"]
pub type MUNSTKERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUNSTKERR`"]
pub struct MUNSTKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MUNSTKERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `MSTKERR`"]
pub type MSTKERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSTKERR`"]
pub struct MSTKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTKERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `MMARVALID`"]
pub type MMARVALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MMARVALID`"]
pub struct MMARVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> MMARVALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IACCVIOL"]
    #[inline(always)]
    pub fn iaccviol(&self) -> IACCVIOL_R {
        IACCVIOL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DACCVIOL"]
    #[inline(always)]
    pub fn daccviol(&self) -> DACCVIOL_R {
        DACCVIOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MUNSTKERR"]
    #[inline(always)]
    pub fn munstkerr(&self) -> MUNSTKERR_R {
        MUNSTKERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MSTKERR"]
    #[inline(always)]
    pub fn mstkerr(&self) -> MSTKERR_R {
        MSTKERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MMARVALID"]
    #[inline(always)]
    pub fn mmarvalid(&self) -> MMARVALID_R {
        MMARVALID_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IACCVIOL"]
    #[inline(always)]
    pub fn iaccviol(&mut self) -> IACCVIOL_W {
        IACCVIOL_W { w: self }
    }
    #[doc = "Bit 1 - DACCVIOL"]
    #[inline(always)]
    pub fn daccviol(&mut self) -> DACCVIOL_W {
        DACCVIOL_W { w: self }
    }
    #[doc = "Bit 3 - MUNSTKERR"]
    #[inline(always)]
    pub fn munstkerr(&mut self) -> MUNSTKERR_W {
        MUNSTKERR_W { w: self }
    }
    #[doc = "Bit 4 - MSTKERR"]
    #[inline(always)]
    pub fn mstkerr(&mut self) -> MSTKERR_W {
        MSTKERR_W { w: self }
    }
    #[doc = "Bit 7 - MMARVALID"]
    #[inline(always)]
    pub fn mmarvalid(&mut self) -> MMARVALID_W {
        MMARVALID_W { w: self }
    }
}

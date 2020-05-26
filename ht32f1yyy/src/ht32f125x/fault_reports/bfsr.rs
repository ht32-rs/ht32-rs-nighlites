#[doc = "Reader of register BFSR"]
pub type R = crate::R<u8, super::BFSR>;
#[doc = "Writer for register BFSR"]
pub type W = crate::W<u8, super::BFSR>;
#[doc = "Register BFSR `reset()`'s with value 0"]
impl crate::ResetValue for super::BFSR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IBUSERR`"]
pub type IBUSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IBUSERR`"]
pub struct IBUSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> IBUSERR_W<'a> {
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
#[doc = "Reader of field `PRECISERR`"]
pub type PRECISERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRECISERR`"]
pub struct PRECISERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRECISERR_W<'a> {
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
#[doc = "Reader of field `IMPRECISERR`"]
pub type IMPRECISERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMPRECISERR`"]
pub struct IMPRECISERR_W<'a> {
    w: &'a mut W,
}
impl<'a> IMPRECISERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `UNSTKERR`"]
pub type UNSTKERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNSTKERR`"]
pub struct UNSTKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> UNSTKERR_W<'a> {
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
#[doc = "Reader of field `STKERR`"]
pub type STKERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STKERR`"]
pub struct STKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> STKERR_W<'a> {
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
#[doc = "Reader of field `BFARVALID`"]
pub type BFARVALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BFARVALID`"]
pub struct BFARVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> BFARVALID_W<'a> {
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
    #[doc = "Bit 0 - IBUSERR"]
    #[inline(always)]
    pub fn ibuserr(&self) -> IBUSERR_R {
        IBUSERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PRECISERR"]
    #[inline(always)]
    pub fn preciserr(&self) -> PRECISERR_R {
        PRECISERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IMPRECISERR"]
    #[inline(always)]
    pub fn impreciserr(&self) -> IMPRECISERR_R {
        IMPRECISERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UNSTKERR"]
    #[inline(always)]
    pub fn unstkerr(&self) -> UNSTKERR_R {
        UNSTKERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - STKERR"]
    #[inline(always)]
    pub fn stkerr(&self) -> STKERR_R {
        STKERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BFARVALID"]
    #[inline(always)]
    pub fn bfarvalid(&self) -> BFARVALID_R {
        BFARVALID_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IBUSERR"]
    #[inline(always)]
    pub fn ibuserr(&mut self) -> IBUSERR_W {
        IBUSERR_W { w: self }
    }
    #[doc = "Bit 1 - PRECISERR"]
    #[inline(always)]
    pub fn preciserr(&mut self) -> PRECISERR_W {
        PRECISERR_W { w: self }
    }
    #[doc = "Bit 2 - IMPRECISERR"]
    #[inline(always)]
    pub fn impreciserr(&mut self) -> IMPRECISERR_W {
        IMPRECISERR_W { w: self }
    }
    #[doc = "Bit 3 - UNSTKERR"]
    #[inline(always)]
    pub fn unstkerr(&mut self) -> UNSTKERR_W {
        UNSTKERR_W { w: self }
    }
    #[doc = "Bit 4 - STKERR"]
    #[inline(always)]
    pub fn stkerr(&mut self) -> STKERR_W {
        STKERR_W { w: self }
    }
    #[doc = "Bit 7 - BFARVALID"]
    #[inline(always)]
    pub fn bfarvalid(&mut self) -> BFARVALID_W {
        BFARVALID_W { w: self }
    }
}

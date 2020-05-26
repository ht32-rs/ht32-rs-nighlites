#[doc = "Reader of register CKCU_MCUDBGCR"]
pub type R = crate::R<u32, super::CKCU_MCUDBGCR>;
#[doc = "Writer for register CKCU_MCUDBGCR"]
pub type W = crate::W<u32, super::CKCU_MCUDBGCR>;
#[doc = "Register CKCU_MCUDBGCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CKCU_MCUDBGCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBSLP`"]
pub type DBSLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBSLP`"]
pub struct DBSLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSLP_W<'a> {
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
#[doc = "Reader of field `DBDSLP1`"]
pub type DBDSLP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBDSLP1`"]
pub struct DBDSLP1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBDSLP1_W<'a> {
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
#[doc = "Reader of field `DBPD`"]
pub type DBPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBPD`"]
pub struct DBPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBPD_W<'a> {
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
#[doc = "Reader of field `DBWDT`"]
pub type DBWDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBWDT`"]
pub struct DBWDT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBWDT_W<'a> {
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
#[doc = "Reader of field `DBGPTM0`"]
pub type DBGPTM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGPTM0`"]
pub struct DBGPTM0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGPTM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DBGPTM1`"]
pub type DBGPTM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGPTM1`"]
pub struct DBGPTM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGPTM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `DBUSART`"]
pub type DBUSART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBUSART`"]
pub struct DBUSART_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUSART_W<'a> {
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
#[doc = "Reader of field `DBSPI`"]
pub type DBSPI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBSPI`"]
pub struct DBSPI_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSPI_W<'a> {
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
#[doc = "Reader of field `DBDSLP2`"]
pub type DBDSLP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBDSLP2`"]
pub struct DBDSLP2_W<'a> {
    w: &'a mut W,
}
impl<'a> DBDSLP2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DBSLP"]
    #[inline(always)]
    pub fn dbslp(&self) -> DBSLP_R {
        DBSLP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DBDSLP1"]
    #[inline(always)]
    pub fn dbdslp1(&self) -> DBDSLP1_R {
        DBDSLP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DBPD"]
    #[inline(always)]
    pub fn dbpd(&self) -> DBPD_R {
        DBPD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DBWDT"]
    #[inline(always)]
    pub fn dbwdt(&self) -> DBWDT_R {
        DBWDT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DBGPTM0"]
    #[inline(always)]
    pub fn dbgptm0(&self) -> DBGPTM0_R {
        DBGPTM0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DBGPTM1"]
    #[inline(always)]
    pub fn dbgptm1(&self) -> DBGPTM1_R {
        DBGPTM1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DBUSART"]
    #[inline(always)]
    pub fn dbusart(&self) -> DBUSART_R {
        DBUSART_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DBSPI"]
    #[inline(always)]
    pub fn dbspi(&self) -> DBSPI_R {
        DBSPI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DBDSLP2"]
    #[inline(always)]
    pub fn dbdslp2(&self) -> DBDSLP2_R {
        DBDSLP2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBSLP"]
    #[inline(always)]
    pub fn dbslp(&mut self) -> DBSLP_W {
        DBSLP_W { w: self }
    }
    #[doc = "Bit 1 - DBDSLP1"]
    #[inline(always)]
    pub fn dbdslp1(&mut self) -> DBDSLP1_W {
        DBDSLP1_W { w: self }
    }
    #[doc = "Bit 2 - DBPD"]
    #[inline(always)]
    pub fn dbpd(&mut self) -> DBPD_W {
        DBPD_W { w: self }
    }
    #[doc = "Bit 3 - DBWDT"]
    #[inline(always)]
    pub fn dbwdt(&mut self) -> DBWDT_W {
        DBWDT_W { w: self }
    }
    #[doc = "Bit 6 - DBGPTM0"]
    #[inline(always)]
    pub fn dbgptm0(&mut self) -> DBGPTM0_W {
        DBGPTM0_W { w: self }
    }
    #[doc = "Bit 7 - DBGPTM1"]
    #[inline(always)]
    pub fn dbgptm1(&mut self) -> DBGPTM1_W {
        DBGPTM1_W { w: self }
    }
    #[doc = "Bit 8 - DBUSART"]
    #[inline(always)]
    pub fn dbusart(&mut self) -> DBUSART_W {
        DBUSART_W { w: self }
    }
    #[doc = "Bit 10 - DBSPI"]
    #[inline(always)]
    pub fn dbspi(&mut self) -> DBSPI_W {
        DBSPI_W { w: self }
    }
    #[doc = "Bit 14 - DBDSLP2"]
    #[inline(always)]
    pub fn dbdslp2(&mut self) -> DBDSLP2_W {
        DBDSLP2_W { w: self }
    }
}

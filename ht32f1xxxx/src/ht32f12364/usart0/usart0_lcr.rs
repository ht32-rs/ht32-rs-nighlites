#[doc = "Reader of register USART0_LCR"]
pub type R = crate::R<u32, super::USART0_LCR>;
#[doc = "Writer for register USART0_LCR"]
pub type W = crate::W<u32, super::USART0_LCR>;
#[doc = "Register USART0_LCR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART0_LCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WLS`"]
pub type WLS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WLS`"]
pub struct WLS_W<'a> {
    w: &'a mut W,
}
impl<'a> WLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `NSB`"]
pub type NSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSB`"]
pub struct NSB_W<'a> {
    w: &'a mut W,
}
impl<'a> NSB_W<'a> {
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
#[doc = "Reader of field `PBE`"]
pub type PBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PBE`"]
pub struct PBE_W<'a> {
    w: &'a mut W,
}
impl<'a> PBE_W<'a> {
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
#[doc = "Reader of field `EPE`"]
pub type EPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPE`"]
pub struct EPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SPE`"]
pub type SPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPE`"]
pub struct SPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `BCB`"]
pub type BCB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BCB`"]
pub struct BCB_W<'a> {
    w: &'a mut W,
}
impl<'a> BCB_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - WLS"]
    #[inline(always)]
    pub fn wls(&self) -> WLS_R {
        WLS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - NSB"]
    #[inline(always)]
    pub fn nsb(&self) -> NSB_R {
        NSB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PBE"]
    #[inline(always)]
    pub fn pbe(&self) -> PBE_R {
        PBE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EPE"]
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPE"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BCB"]
    #[inline(always)]
    pub fn bcb(&self) -> BCB_R {
        BCB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - WLS"]
    #[inline(always)]
    pub fn wls(&mut self) -> WLS_W {
        WLS_W { w: self }
    }
    #[doc = "Bit 2 - NSB"]
    #[inline(always)]
    pub fn nsb(&mut self) -> NSB_W {
        NSB_W { w: self }
    }
    #[doc = "Bit 3 - PBE"]
    #[inline(always)]
    pub fn pbe(&mut self) -> PBE_W {
        PBE_W { w: self }
    }
    #[doc = "Bit 4 - EPE"]
    #[inline(always)]
    pub fn epe(&mut self) -> EPE_W {
        EPE_W { w: self }
    }
    #[doc = "Bit 5 - SPE"]
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W {
        SPE_W { w: self }
    }
    #[doc = "Bit 6 - BCB"]
    #[inline(always)]
    pub fn bcb(&mut self) -> BCB_W {
        BCB_W { w: self }
    }
}

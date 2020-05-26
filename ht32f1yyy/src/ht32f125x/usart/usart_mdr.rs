#[doc = "Reader of register USART_MDR"]
pub type R = crate::R<u32, super::USART_MDR>;
#[doc = "Writer for register USART_MDR"]
pub type W = crate::W<u32, super::USART_MDR>;
#[doc = "Register USART_MDR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART_MDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TRSM`"]
pub type TRSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRSM`"]
pub struct TRSM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRSM_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - TRSM"]
    #[inline(always)]
    pub fn trsm(&self) -> TRSM_R {
        TRSM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 2 - TRSM"]
    #[inline(always)]
    pub fn trsm(&mut self) -> TRSM_W {
        TRSM_W { w: self }
    }
}

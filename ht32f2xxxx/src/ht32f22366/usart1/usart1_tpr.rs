#[doc = "Reader of register USART1_TPR"]
pub type R = crate::R<u32, super::USART1_TPR>;
#[doc = "Writer for register USART1_TPR"]
pub type W = crate::W<u32, super::USART1_TPR>;
#[doc = "Register USART1_TPR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART1_TPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTOIC`"]
pub type RTOIC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTOIC`"]
pub struct RTOIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTOIC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `RTOIE`"]
pub type RTOIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTOIE`"]
pub struct RTOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTOIE_W<'a> {
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
#[doc = "Reader of field `TG`"]
pub type TG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TG`"]
pub struct TG_W<'a> {
    w: &'a mut W,
}
impl<'a> TG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - RTOIC"]
    #[inline(always)]
    pub fn rtoic(&self) -> RTOIC_R {
        RTOIC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - RTOIE"]
    #[inline(always)]
    pub fn rtoie(&self) -> RTOIE_R {
        RTOIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - TG"]
    #[inline(always)]
    pub fn tg(&self) -> TG_R {
        TG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - RTOIC"]
    #[inline(always)]
    pub fn rtoic(&mut self) -> RTOIC_W {
        RTOIC_W { w: self }
    }
    #[doc = "Bit 7 - RTOIE"]
    #[inline(always)]
    pub fn rtoie(&mut self) -> RTOIE_W {
        RTOIE_W { w: self }
    }
    #[doc = "Bits 8:15 - TG"]
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W {
        TG_W { w: self }
    }
}

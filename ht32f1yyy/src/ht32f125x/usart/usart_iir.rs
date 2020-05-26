#[doc = "Reader of register USART_IIR"]
pub type R = crate::R<u32, super::USART_IIR>;
#[doc = "Writer for register USART_IIR"]
pub type W = crate::W<u32, super::USART_IIR>;
#[doc = "Register USART_IIR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART_IIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NIP`"]
pub type NIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NIP`"]
pub struct NIP_W<'a> {
    w: &'a mut W,
}
impl<'a> NIP_W<'a> {
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
#[doc = "Reader of field `IID`"]
pub type IID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IID`"]
pub struct IID_W<'a> {
    w: &'a mut W,
}
impl<'a> IID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - NIP"]
    #[inline(always)]
    pub fn nip(&self) -> NIP_R {
        NIP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - IID"]
    #[inline(always)]
    pub fn iid(&self) -> IID_R {
        IID_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - NIP"]
    #[inline(always)]
    pub fn nip(&mut self) -> NIP_W {
        NIP_W { w: self }
    }
    #[doc = "Bits 1:3 - IID"]
    #[inline(always)]
    pub fn iid(&mut self) -> IID_W {
        IID_W { w: self }
    }
}

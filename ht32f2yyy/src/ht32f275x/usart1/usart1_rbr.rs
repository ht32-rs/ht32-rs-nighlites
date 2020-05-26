#[doc = "Reader of register USART1_RBR"]
pub type R = crate::R<u32, super::USART1_RBR>;
#[doc = "Writer for register USART1_RBR"]
pub type W = crate::W<u32, super::USART1_RBR>;
#[doc = "Register USART1_RBR `reset()`'s with value 0"]
impl crate::ResetValue for super::USART1_RBR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RD`"]
pub type RD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RD`"]
pub struct RD_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - RD"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - RD"]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W {
        RD_W { w: self }
    }
}

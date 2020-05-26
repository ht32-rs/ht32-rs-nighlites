#[doc = "Reader of register UART0_TBR"]
pub type R = crate::R<u32, super::UART0_TBR>;
#[doc = "Writer for register UART0_TBR"]
pub type W = crate::W<u32, super::UART0_TBR>;
#[doc = "Register UART0_TBR `reset()`'s with value 0"]
impl crate::ResetValue for super::UART0_TBR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TD`"]
pub type TD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TD`"]
pub struct TD_W<'a> {
    w: &'a mut W,
}
impl<'a> TD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - TD"]
    #[inline(always)]
    pub fn td(&self) -> TD_R {
        TD_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - TD"]
    #[inline(always)]
    pub fn td(&mut self) -> TD_W {
        TD_W { w: self }
    }
}

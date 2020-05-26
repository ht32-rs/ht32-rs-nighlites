#[doc = "Reader of register OFVCR0"]
pub type R = crate::R<u32, super::OFVCR0>;
#[doc = "Writer for register OFVCR0"]
pub type W = crate::W<u32, super::OFVCR0>;
#[doc = "Register OFVCR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::OFVCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `A0OF`"]
pub type A0OF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `A0OF`"]
pub struct A0OF_W<'a> {
    w: &'a mut W,
}
impl<'a> A0OF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - A0OF"]
    #[inline(always)]
    pub fn a0of(&self) -> A0OF_R {
        A0OF_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - A0OF"]
    #[inline(always)]
    pub fn a0of(&mut self) -> A0OF_W {
        A0OF_W { w: self }
    }
}

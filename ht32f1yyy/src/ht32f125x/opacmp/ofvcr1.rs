#[doc = "Reader of register OFVCR1"]
pub type R = crate::R<u32, super::OFVCR1>;
#[doc = "Writer for register OFVCR1"]
pub type W = crate::W<u32, super::OFVCR1>;
#[doc = "Register OFVCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::OFVCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `A1OF`"]
pub type A1OF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `A1OF`"]
pub struct A1OF_W<'a> {
    w: &'a mut W,
}
impl<'a> A1OF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - A1OF"]
    #[inline(always)]
    pub fn a1of(&self) -> A1OF_R {
        A1OF_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - A1OF"]
    #[inline(always)]
    pub fn a1of(&mut self) -> A1OF_W {
        A1OF_W { w: self }
    }
}

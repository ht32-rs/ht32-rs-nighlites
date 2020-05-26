#[doc = "Reader of register PWRCU_HSIRCR"]
pub type R = crate::R<u32, super::PWRCU_HSIRCR>;
#[doc = "Writer for register PWRCU_HSIRCR"]
pub type W = crate::W<u32, super::PWRCU_HSIRCR>;
#[doc = "Register PWRCU_HSIRCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRCU_HSIRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSIRCBL`"]
pub type HSIRCBL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSIRCBL`"]
pub struct HSIRCBL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRCBL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - HSIRCBL"]
    #[inline(always)]
    pub fn hsircbl(&self) -> HSIRCBL_R {
        HSIRCBL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - HSIRCBL"]
    #[inline(always)]
    pub fn hsircbl(&mut self) -> HSIRCBL_W {
        HSIRCBL_W { w: self }
    }
}

#[doc = "Reader of register I2C_ADDSR"]
pub type R = crate::R<u32, super::I2C_ADDSR>;
#[doc = "Writer for register I2C_ADDSR"]
pub type W = crate::W<u32, super::I2C_ADDSR>;
#[doc = "Register I2C_ADDSR `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_ADDSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDSR`"]
pub type ADDSR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDSR`"]
pub struct ADDSR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - ADDSR"]
    #[inline(always)]
    pub fn addsr(&self) -> ADDSR_R {
        ADDSR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - ADDSR"]
    #[inline(always)]
    pub fn addsr(&mut self) -> ADDSR_W {
        ADDSR_W { w: self }
    }
}

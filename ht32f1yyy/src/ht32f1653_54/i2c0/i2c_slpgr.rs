#[doc = "Reader of register I2C_SLPGR"]
pub type R = crate::R<u32, super::I2C_SLPGR>;
#[doc = "Writer for register I2C_SLPGR"]
pub type W = crate::W<u32, super::I2C_SLPGR>;
#[doc = "Register I2C_SLPGR `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_SLPGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLPG`"]
pub type SLPG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLPG`"]
pub struct SLPG_W<'a> {
    w: &'a mut W,
}
impl<'a> SLPG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - SLPG"]
    #[inline(always)]
    pub fn slpg(&self) -> SLPG_R {
        SLPG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SLPG"]
    #[inline(always)]
    pub fn slpg(&mut self) -> SLPG_W {
        SLPG_W { w: self }
    }
}

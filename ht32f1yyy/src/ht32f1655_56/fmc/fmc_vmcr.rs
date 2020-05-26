#[doc = "Reader of register FMC_VMCR"]
pub type R = crate::R<u32, super::FMC_VMCR>;
#[doc = "Writer for register FMC_VMCR"]
pub type W = crate::W<u32, super::FMC_VMCR>;
#[doc = "Register FMC_VMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FMC_VMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VMCB`"]
pub type VMCB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VMCB`"]
pub struct VMCB_W<'a> {
    w: &'a mut W,
}
impl<'a> VMCB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - VMCB"]
    #[inline(always)]
    pub fn vmcb(&self) -> VMCB_R {
        VMCB_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - VMCB"]
    #[inline(always)]
    pub fn vmcb(&mut self) -> VMCB_W {
        VMCB_W { w: self }
    }
}

#[doc = "Reader of register EBI_IFCR"]
pub type R = crate::R<u32, super::EBI_IFCR>;
#[doc = "Writer for register EBI_IFCR"]
pub type W = crate::W<u32, super::EBI_IFCR>;
#[doc = "Register EBI_IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::EBI_IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ARDYTOIC`"]
pub type ARDYTOIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARDYTOIC`"]
pub struct ARDYTOIC_W<'a> {
    w: &'a mut W,
}
impl<'a> ARDYTOIC_W<'a> {
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
#[doc = "Reader of field `ACCERRIC`"]
pub type ACCERRIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCERRIC`"]
pub struct ACCERRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCERRIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ARDYTOIC"]
    #[inline(always)]
    pub fn ardytoic(&self) -> ARDYTOIC_R {
        ARDYTOIC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ACCERRIC"]
    #[inline(always)]
    pub fn accerric(&self) -> ACCERRIC_R {
        ACCERRIC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARDYTOIC"]
    #[inline(always)]
    pub fn ardytoic(&mut self) -> ARDYTOIC_W {
        ARDYTOIC_W { w: self }
    }
    #[doc = "Bit 1 - ACCERRIC"]
    #[inline(always)]
    pub fn accerric(&mut self) -> ACCERRIC_W {
        ACCERRIC_W { w: self }
    }
}

#[doc = "Reader of register EBI_IENR"]
pub type R = crate::R<u32, super::EBI_IENR>;
#[doc = "Writer for register EBI_IENR"]
pub type W = crate::W<u32, super::EBI_IENR>;
#[doc = "Register EBI_IENR `reset()`'s with value 0"]
impl crate::ResetValue for super::EBI_IENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ARDYTOIEN`"]
pub type ARDYTOIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARDYTOIEN`"]
pub struct ARDYTOIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARDYTOIEN_W<'a> {
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
#[doc = "Reader of field `ACCDISIEN`"]
pub type ACCDISIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCDISIEN`"]
pub struct ACCDISIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCDISIEN_W<'a> {
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
#[doc = "Reader of field `ACCRSTIEN`"]
pub type ACCRSTIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACCRSTIEN`"]
pub struct ACCRSTIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCRSTIEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ARDYTOIEN"]
    #[inline(always)]
    pub fn ardytoien(&self) -> ARDYTOIEN_R {
        ARDYTOIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ACCDISIEN"]
    #[inline(always)]
    pub fn accdisien(&self) -> ACCDISIEN_R {
        ACCDISIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ACCRSTIEN"]
    #[inline(always)]
    pub fn accrstien(&self) -> ACCRSTIEN_R {
        ACCRSTIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARDYTOIEN"]
    #[inline(always)]
    pub fn ardytoien(&mut self) -> ARDYTOIEN_W {
        ARDYTOIEN_W { w: self }
    }
    #[doc = "Bit 1 - ACCDISIEN"]
    #[inline(always)]
    pub fn accdisien(&mut self) -> ACCDISIEN_W {
        ACCDISIEN_W { w: self }
    }
    #[doc = "Bit 2 - ACCRSTIEN"]
    #[inline(always)]
    pub fn accrstien(&mut self) -> ACCRSTIEN_W {
        ACCRSTIEN_W { w: self }
    }
}

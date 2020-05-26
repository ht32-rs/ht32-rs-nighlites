#[doc = "Reader of register PDMA_IER1"]
pub type R = crate::R<u32, super::PDMA_IER1>;
#[doc = "Writer for register PDMA_IER1"]
pub type W = crate::W<u32, super::PDMA_IER1>;
#[doc = "Register PDMA_IER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PDMA_IER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GEIE6`"]
pub type GEIE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEIE6`"]
pub struct GEIE6_W<'a> {
    w: &'a mut W,
}
impl<'a> GEIE6_W<'a> {
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
#[doc = "Reader of field `BEIE6`"]
pub type BEIE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIE6`"]
pub struct BEIE6_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIE6_W<'a> {
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
#[doc = "Reader of field `HTIE6`"]
pub type HTIE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTIE6`"]
pub struct HTIE6_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIE6_W<'a> {
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
#[doc = "Reader of field `TCIE6`"]
pub type TCIE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCIE6`"]
pub struct TCIE6_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TEIE6`"]
pub type TEIE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEIE6`"]
pub struct TEIE6_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `GEIE7`"]
pub type GEIE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEIE7`"]
pub struct GEIE7_W<'a> {
    w: &'a mut W,
}
impl<'a> GEIE7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `BEIE7`"]
pub type BEIE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIE7`"]
pub struct BEIE7_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIE7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `HTIE7`"]
pub type HTIE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTIE7`"]
pub struct HTIE7_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIE7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TCIE7`"]
pub type TCIE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCIE7`"]
pub struct TCIE7_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TEIE7`"]
pub type TEIE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEIE7`"]
pub struct TEIE7_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `GEIE8`"]
pub type GEIE8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEIE8`"]
pub struct GEIE8_W<'a> {
    w: &'a mut W,
}
impl<'a> GEIE8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `BEIE8`"]
pub type BEIE8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIE8`"]
pub struct BEIE8_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIE8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `HTIE8`"]
pub type HTIE8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTIE8`"]
pub struct HTIE8_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIE8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TCIE8`"]
pub type TCIE8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCIE8`"]
pub struct TCIE8_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `TEIE8`"]
pub type TEIE8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEIE8`"]
pub struct TEIE8_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `GEIE9`"]
pub type GEIE9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEIE9`"]
pub struct GEIE9_W<'a> {
    w: &'a mut W,
}
impl<'a> GEIE9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `BEIE9`"]
pub type BEIE9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIE9`"]
pub struct BEIE9_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIE9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `HTIE9`"]
pub type HTIE9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTIE9`"]
pub struct HTIE9_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIE9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `TCIE9`"]
pub type TCIE9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCIE9`"]
pub struct TCIE9_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `TEIE9`"]
pub type TEIE9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEIE9`"]
pub struct TEIE9_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `GEIE10`"]
pub type GEIE10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEIE10`"]
pub struct GEIE10_W<'a> {
    w: &'a mut W,
}
impl<'a> GEIE10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `BEIE10`"]
pub type BEIE10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIE10`"]
pub struct BEIE10_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIE10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `HTIE10`"]
pub type HTIE10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTIE10`"]
pub struct HTIE10_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIE10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `TCIE10`"]
pub type TCIE10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCIE10`"]
pub struct TCIE10_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `TEIE10`"]
pub type TEIE10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEIE10`"]
pub struct TEIE10_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `GEIE11`"]
pub type GEIE11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEIE11`"]
pub struct GEIE11_W<'a> {
    w: &'a mut W,
}
impl<'a> GEIE11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `BEIE11`"]
pub type BEIE11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIE11`"]
pub struct BEIE11_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIE11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `HTIE11`"]
pub type HTIE11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTIE11`"]
pub struct HTIE11_W<'a> {
    w: &'a mut W,
}
impl<'a> HTIE11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `TCIE11`"]
pub type TCIE11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCIE11`"]
pub struct TCIE11_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `TEIE11`"]
pub type TEIE11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEIE11`"]
pub struct TEIE11_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GEIE6"]
    #[inline(always)]
    pub fn geie6(&self) -> GEIE6_R {
        GEIE6_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BEIE6"]
    #[inline(always)]
    pub fn beie6(&self) -> BEIE6_R {
        BEIE6_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HTIE6"]
    #[inline(always)]
    pub fn htie6(&self) -> HTIE6_R {
        HTIE6_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TCIE6"]
    #[inline(always)]
    pub fn tcie6(&self) -> TCIE6_R {
        TCIE6_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TEIE6"]
    #[inline(always)]
    pub fn teie6(&self) -> TEIE6_R {
        TEIE6_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GEIE7"]
    #[inline(always)]
    pub fn geie7(&self) -> GEIE7_R {
        GEIE7_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BEIE7"]
    #[inline(always)]
    pub fn beie7(&self) -> BEIE7_R {
        BEIE7_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - HTIE7"]
    #[inline(always)]
    pub fn htie7(&self) -> HTIE7_R {
        HTIE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TCIE7"]
    #[inline(always)]
    pub fn tcie7(&self) -> TCIE7_R {
        TCIE7_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TEIE7"]
    #[inline(always)]
    pub fn teie7(&self) -> TEIE7_R {
        TEIE7_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GEIE8"]
    #[inline(always)]
    pub fn geie8(&self) -> GEIE8_R {
        GEIE8_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BEIE8"]
    #[inline(always)]
    pub fn beie8(&self) -> BEIE8_R {
        BEIE8_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - HTIE8"]
    #[inline(always)]
    pub fn htie8(&self) -> HTIE8_R {
        HTIE8_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TCIE8"]
    #[inline(always)]
    pub fn tcie8(&self) -> TCIE8_R {
        TCIE8_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TEIE8"]
    #[inline(always)]
    pub fn teie8(&self) -> TEIE8_R {
        TEIE8_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GEIE9"]
    #[inline(always)]
    pub fn geie9(&self) -> GEIE9_R {
        GEIE9_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BEIE9"]
    #[inline(always)]
    pub fn beie9(&self) -> BEIE9_R {
        BEIE9_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HTIE9"]
    #[inline(always)]
    pub fn htie9(&self) -> HTIE9_R {
        HTIE9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TCIE9"]
    #[inline(always)]
    pub fn tcie9(&self) -> TCIE9_R {
        TCIE9_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TEIE9"]
    #[inline(always)]
    pub fn teie9(&self) -> TEIE9_R {
        TEIE9_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - GEIE10"]
    #[inline(always)]
    pub fn geie10(&self) -> GEIE10_R {
        GEIE10_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BEIE10"]
    #[inline(always)]
    pub fn beie10(&self) -> BEIE10_R {
        BEIE10_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - HTIE10"]
    #[inline(always)]
    pub fn htie10(&self) -> HTIE10_R {
        HTIE10_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TCIE10"]
    #[inline(always)]
    pub fn tcie10(&self) -> TCIE10_R {
        TCIE10_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TEIE10"]
    #[inline(always)]
    pub fn teie10(&self) -> TEIE10_R {
        TEIE10_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - GEIE11"]
    #[inline(always)]
    pub fn geie11(&self) -> GEIE11_R {
        GEIE11_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - BEIE11"]
    #[inline(always)]
    pub fn beie11(&self) -> BEIE11_R {
        BEIE11_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - HTIE11"]
    #[inline(always)]
    pub fn htie11(&self) -> HTIE11_R {
        HTIE11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TCIE11"]
    #[inline(always)]
    pub fn tcie11(&self) -> TCIE11_R {
        TCIE11_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TEIE11"]
    #[inline(always)]
    pub fn teie11(&self) -> TEIE11_R {
        TEIE11_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GEIE6"]
    #[inline(always)]
    pub fn geie6(&mut self) -> GEIE6_W {
        GEIE6_W { w: self }
    }
    #[doc = "Bit 1 - BEIE6"]
    #[inline(always)]
    pub fn beie6(&mut self) -> BEIE6_W {
        BEIE6_W { w: self }
    }
    #[doc = "Bit 2 - HTIE6"]
    #[inline(always)]
    pub fn htie6(&mut self) -> HTIE6_W {
        HTIE6_W { w: self }
    }
    #[doc = "Bit 3 - TCIE6"]
    #[inline(always)]
    pub fn tcie6(&mut self) -> TCIE6_W {
        TCIE6_W { w: self }
    }
    #[doc = "Bit 4 - TEIE6"]
    #[inline(always)]
    pub fn teie6(&mut self) -> TEIE6_W {
        TEIE6_W { w: self }
    }
    #[doc = "Bit 5 - GEIE7"]
    #[inline(always)]
    pub fn geie7(&mut self) -> GEIE7_W {
        GEIE7_W { w: self }
    }
    #[doc = "Bit 6 - BEIE7"]
    #[inline(always)]
    pub fn beie7(&mut self) -> BEIE7_W {
        BEIE7_W { w: self }
    }
    #[doc = "Bit 7 - HTIE7"]
    #[inline(always)]
    pub fn htie7(&mut self) -> HTIE7_W {
        HTIE7_W { w: self }
    }
    #[doc = "Bit 8 - TCIE7"]
    #[inline(always)]
    pub fn tcie7(&mut self) -> TCIE7_W {
        TCIE7_W { w: self }
    }
    #[doc = "Bit 9 - TEIE7"]
    #[inline(always)]
    pub fn teie7(&mut self) -> TEIE7_W {
        TEIE7_W { w: self }
    }
    #[doc = "Bit 10 - GEIE8"]
    #[inline(always)]
    pub fn geie8(&mut self) -> GEIE8_W {
        GEIE8_W { w: self }
    }
    #[doc = "Bit 11 - BEIE8"]
    #[inline(always)]
    pub fn beie8(&mut self) -> BEIE8_W {
        BEIE8_W { w: self }
    }
    #[doc = "Bit 12 - HTIE8"]
    #[inline(always)]
    pub fn htie8(&mut self) -> HTIE8_W {
        HTIE8_W { w: self }
    }
    #[doc = "Bit 13 - TCIE8"]
    #[inline(always)]
    pub fn tcie8(&mut self) -> TCIE8_W {
        TCIE8_W { w: self }
    }
    #[doc = "Bit 14 - TEIE8"]
    #[inline(always)]
    pub fn teie8(&mut self) -> TEIE8_W {
        TEIE8_W { w: self }
    }
    #[doc = "Bit 15 - GEIE9"]
    #[inline(always)]
    pub fn geie9(&mut self) -> GEIE9_W {
        GEIE9_W { w: self }
    }
    #[doc = "Bit 16 - BEIE9"]
    #[inline(always)]
    pub fn beie9(&mut self) -> BEIE9_W {
        BEIE9_W { w: self }
    }
    #[doc = "Bit 17 - HTIE9"]
    #[inline(always)]
    pub fn htie9(&mut self) -> HTIE9_W {
        HTIE9_W { w: self }
    }
    #[doc = "Bit 18 - TCIE9"]
    #[inline(always)]
    pub fn tcie9(&mut self) -> TCIE9_W {
        TCIE9_W { w: self }
    }
    #[doc = "Bit 19 - TEIE9"]
    #[inline(always)]
    pub fn teie9(&mut self) -> TEIE9_W {
        TEIE9_W { w: self }
    }
    #[doc = "Bit 20 - GEIE10"]
    #[inline(always)]
    pub fn geie10(&mut self) -> GEIE10_W {
        GEIE10_W { w: self }
    }
    #[doc = "Bit 21 - BEIE10"]
    #[inline(always)]
    pub fn beie10(&mut self) -> BEIE10_W {
        BEIE10_W { w: self }
    }
    #[doc = "Bit 22 - HTIE10"]
    #[inline(always)]
    pub fn htie10(&mut self) -> HTIE10_W {
        HTIE10_W { w: self }
    }
    #[doc = "Bit 23 - TCIE10"]
    #[inline(always)]
    pub fn tcie10(&mut self) -> TCIE10_W {
        TCIE10_W { w: self }
    }
    #[doc = "Bit 24 - TEIE10"]
    #[inline(always)]
    pub fn teie10(&mut self) -> TEIE10_W {
        TEIE10_W { w: self }
    }
    #[doc = "Bit 25 - GEIE11"]
    #[inline(always)]
    pub fn geie11(&mut self) -> GEIE11_W {
        GEIE11_W { w: self }
    }
    #[doc = "Bit 26 - BEIE11"]
    #[inline(always)]
    pub fn beie11(&mut self) -> BEIE11_W {
        BEIE11_W { w: self }
    }
    #[doc = "Bit 27 - HTIE11"]
    #[inline(always)]
    pub fn htie11(&mut self) -> HTIE11_W {
        HTIE11_W { w: self }
    }
    #[doc = "Bit 28 - TCIE11"]
    #[inline(always)]
    pub fn tcie11(&mut self) -> TCIE11_W {
        TCIE11_W { w: self }
    }
    #[doc = "Bit 29 - TEIE11"]
    #[inline(always)]
    pub fn teie11(&mut self) -> TEIE11_W {
        TEIE11_W { w: self }
    }
}

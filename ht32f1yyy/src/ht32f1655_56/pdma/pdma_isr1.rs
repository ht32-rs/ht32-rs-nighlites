#[doc = "Reader of register PDMA_ISR1"]
pub type R = crate::R<u32, super::PDMA_ISR1>;
#[doc = "Writer for register PDMA_ISR1"]
pub type W = crate::W<u32, super::PDMA_ISR1>;
#[doc = "Register PDMA_ISR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PDMA_ISR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GEISTA6`"]
pub type GEISTA6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEISTA6`"]
pub struct GEISTA6_W<'a> {
    w: &'a mut W,
}
impl<'a> GEISTA6_W<'a> {
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
#[doc = "Reader of field `BEISTA6`"]
pub type BEISTA6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEISTA6`"]
pub struct BEISTA6_W<'a> {
    w: &'a mut W,
}
impl<'a> BEISTA6_W<'a> {
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
#[doc = "Reader of field `HTISTA6`"]
pub type HTISTA6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTISTA6`"]
pub struct HTISTA6_W<'a> {
    w: &'a mut W,
}
impl<'a> HTISTA6_W<'a> {
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
#[doc = "Reader of field `TCISTA6`"]
pub type TCISTA6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCISTA6`"]
pub struct TCISTA6_W<'a> {
    w: &'a mut W,
}
impl<'a> TCISTA6_W<'a> {
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
#[doc = "Reader of field `TEISTA6`"]
pub type TEISTA6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEISTA6`"]
pub struct TEISTA6_W<'a> {
    w: &'a mut W,
}
impl<'a> TEISTA6_W<'a> {
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
#[doc = "Reader of field `GEISTA7`"]
pub type GEISTA7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEISTA7`"]
pub struct GEISTA7_W<'a> {
    w: &'a mut W,
}
impl<'a> GEISTA7_W<'a> {
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
#[doc = "Reader of field `BEISTA7`"]
pub type BEISTA7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEISTA7`"]
pub struct BEISTA7_W<'a> {
    w: &'a mut W,
}
impl<'a> BEISTA7_W<'a> {
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
#[doc = "Reader of field `HTISTA7`"]
pub type HTISTA7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTISTA7`"]
pub struct HTISTA7_W<'a> {
    w: &'a mut W,
}
impl<'a> HTISTA7_W<'a> {
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
#[doc = "Reader of field `TCISTA7`"]
pub type TCISTA7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCISTA7`"]
pub struct TCISTA7_W<'a> {
    w: &'a mut W,
}
impl<'a> TCISTA7_W<'a> {
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
#[doc = "Reader of field `TEISTA7`"]
pub type TEISTA7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEISTA7`"]
pub struct TEISTA7_W<'a> {
    w: &'a mut W,
}
impl<'a> TEISTA7_W<'a> {
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
#[doc = "Reader of field `GEISTA8`"]
pub type GEISTA8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEISTA8`"]
pub struct GEISTA8_W<'a> {
    w: &'a mut W,
}
impl<'a> GEISTA8_W<'a> {
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
#[doc = "Reader of field `BEISTA8`"]
pub type BEISTA8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEISTA8`"]
pub struct BEISTA8_W<'a> {
    w: &'a mut W,
}
impl<'a> BEISTA8_W<'a> {
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
#[doc = "Reader of field `HTISTA8`"]
pub type HTISTA8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTISTA8`"]
pub struct HTISTA8_W<'a> {
    w: &'a mut W,
}
impl<'a> HTISTA8_W<'a> {
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
#[doc = "Reader of field `TCISTA8`"]
pub type TCISTA8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCISTA8`"]
pub struct TCISTA8_W<'a> {
    w: &'a mut W,
}
impl<'a> TCISTA8_W<'a> {
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
#[doc = "Reader of field `TEISTA8`"]
pub type TEISTA8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEISTA8`"]
pub struct TEISTA8_W<'a> {
    w: &'a mut W,
}
impl<'a> TEISTA8_W<'a> {
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
#[doc = "Reader of field `GEISTA9`"]
pub type GEISTA9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEISTA9`"]
pub struct GEISTA9_W<'a> {
    w: &'a mut W,
}
impl<'a> GEISTA9_W<'a> {
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
#[doc = "Reader of field `BEISTA9`"]
pub type BEISTA9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEISTA9`"]
pub struct BEISTA9_W<'a> {
    w: &'a mut W,
}
impl<'a> BEISTA9_W<'a> {
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
#[doc = "Reader of field `HTISTA9`"]
pub type HTISTA9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTISTA9`"]
pub struct HTISTA9_W<'a> {
    w: &'a mut W,
}
impl<'a> HTISTA9_W<'a> {
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
#[doc = "Reader of field `TCISTA9`"]
pub type TCISTA9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCISTA9`"]
pub struct TCISTA9_W<'a> {
    w: &'a mut W,
}
impl<'a> TCISTA9_W<'a> {
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
#[doc = "Reader of field `TEISTA9`"]
pub type TEISTA9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEISTA9`"]
pub struct TEISTA9_W<'a> {
    w: &'a mut W,
}
impl<'a> TEISTA9_W<'a> {
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
#[doc = "Reader of field `GEISTA10`"]
pub type GEISTA10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEISTA10`"]
pub struct GEISTA10_W<'a> {
    w: &'a mut W,
}
impl<'a> GEISTA10_W<'a> {
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
#[doc = "Reader of field `BEISTA10`"]
pub type BEISTA10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEISTA10`"]
pub struct BEISTA10_W<'a> {
    w: &'a mut W,
}
impl<'a> BEISTA10_W<'a> {
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
#[doc = "Reader of field `HTISTA10`"]
pub type HTISTA10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTISTA10`"]
pub struct HTISTA10_W<'a> {
    w: &'a mut W,
}
impl<'a> HTISTA10_W<'a> {
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
#[doc = "Reader of field `TCISTA10`"]
pub type TCISTA10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCISTA10`"]
pub struct TCISTA10_W<'a> {
    w: &'a mut W,
}
impl<'a> TCISTA10_W<'a> {
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
#[doc = "Reader of field `TEISTA10`"]
pub type TEISTA10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEISTA10`"]
pub struct TEISTA10_W<'a> {
    w: &'a mut W,
}
impl<'a> TEISTA10_W<'a> {
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
#[doc = "Reader of field `GEISTA11`"]
pub type GEISTA11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEISTA11`"]
pub struct GEISTA11_W<'a> {
    w: &'a mut W,
}
impl<'a> GEISTA11_W<'a> {
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
#[doc = "Reader of field `BEISTA11`"]
pub type BEISTA11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEISTA11`"]
pub struct BEISTA11_W<'a> {
    w: &'a mut W,
}
impl<'a> BEISTA11_W<'a> {
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
#[doc = "Reader of field `HTISTA11`"]
pub type HTISTA11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTISTA11`"]
pub struct HTISTA11_W<'a> {
    w: &'a mut W,
}
impl<'a> HTISTA11_W<'a> {
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
#[doc = "Reader of field `TCISTA11`"]
pub type TCISTA11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCISTA11`"]
pub struct TCISTA11_W<'a> {
    w: &'a mut W,
}
impl<'a> TCISTA11_W<'a> {
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
#[doc = "Reader of field `TEISTA11`"]
pub type TEISTA11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEISTA11`"]
pub struct TEISTA11_W<'a> {
    w: &'a mut W,
}
impl<'a> TEISTA11_W<'a> {
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
    #[doc = "Bit 0 - GEISTA6"]
    #[inline(always)]
    pub fn geista6(&self) -> GEISTA6_R {
        GEISTA6_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BEISTA6"]
    #[inline(always)]
    pub fn beista6(&self) -> BEISTA6_R {
        BEISTA6_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HTISTA6"]
    #[inline(always)]
    pub fn htista6(&self) -> HTISTA6_R {
        HTISTA6_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TCISTA6"]
    #[inline(always)]
    pub fn tcista6(&self) -> TCISTA6_R {
        TCISTA6_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TEISTA6"]
    #[inline(always)]
    pub fn teista6(&self) -> TEISTA6_R {
        TEISTA6_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GEISTA7"]
    #[inline(always)]
    pub fn geista7(&self) -> GEISTA7_R {
        GEISTA7_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BEISTA7"]
    #[inline(always)]
    pub fn beista7(&self) -> BEISTA7_R {
        BEISTA7_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - HTISTA7"]
    #[inline(always)]
    pub fn htista7(&self) -> HTISTA7_R {
        HTISTA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TCISTA7"]
    #[inline(always)]
    pub fn tcista7(&self) -> TCISTA7_R {
        TCISTA7_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TEISTA7"]
    #[inline(always)]
    pub fn teista7(&self) -> TEISTA7_R {
        TEISTA7_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GEISTA8"]
    #[inline(always)]
    pub fn geista8(&self) -> GEISTA8_R {
        GEISTA8_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BEISTA8"]
    #[inline(always)]
    pub fn beista8(&self) -> BEISTA8_R {
        BEISTA8_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - HTISTA8"]
    #[inline(always)]
    pub fn htista8(&self) -> HTISTA8_R {
        HTISTA8_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TCISTA8"]
    #[inline(always)]
    pub fn tcista8(&self) -> TCISTA8_R {
        TCISTA8_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TEISTA8"]
    #[inline(always)]
    pub fn teista8(&self) -> TEISTA8_R {
        TEISTA8_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GEISTA9"]
    #[inline(always)]
    pub fn geista9(&self) -> GEISTA9_R {
        GEISTA9_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BEISTA9"]
    #[inline(always)]
    pub fn beista9(&self) -> BEISTA9_R {
        BEISTA9_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HTISTA9"]
    #[inline(always)]
    pub fn htista9(&self) -> HTISTA9_R {
        HTISTA9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TCISTA9"]
    #[inline(always)]
    pub fn tcista9(&self) -> TCISTA9_R {
        TCISTA9_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TEISTA9"]
    #[inline(always)]
    pub fn teista9(&self) -> TEISTA9_R {
        TEISTA9_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - GEISTA10"]
    #[inline(always)]
    pub fn geista10(&self) -> GEISTA10_R {
        GEISTA10_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BEISTA10"]
    #[inline(always)]
    pub fn beista10(&self) -> BEISTA10_R {
        BEISTA10_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - HTISTA10"]
    #[inline(always)]
    pub fn htista10(&self) -> HTISTA10_R {
        HTISTA10_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TCISTA10"]
    #[inline(always)]
    pub fn tcista10(&self) -> TCISTA10_R {
        TCISTA10_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TEISTA10"]
    #[inline(always)]
    pub fn teista10(&self) -> TEISTA10_R {
        TEISTA10_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - GEISTA11"]
    #[inline(always)]
    pub fn geista11(&self) -> GEISTA11_R {
        GEISTA11_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - BEISTA11"]
    #[inline(always)]
    pub fn beista11(&self) -> BEISTA11_R {
        BEISTA11_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - HTISTA11"]
    #[inline(always)]
    pub fn htista11(&self) -> HTISTA11_R {
        HTISTA11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TCISTA11"]
    #[inline(always)]
    pub fn tcista11(&self) -> TCISTA11_R {
        TCISTA11_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TEISTA11"]
    #[inline(always)]
    pub fn teista11(&self) -> TEISTA11_R {
        TEISTA11_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GEISTA6"]
    #[inline(always)]
    pub fn geista6(&mut self) -> GEISTA6_W {
        GEISTA6_W { w: self }
    }
    #[doc = "Bit 1 - BEISTA6"]
    #[inline(always)]
    pub fn beista6(&mut self) -> BEISTA6_W {
        BEISTA6_W { w: self }
    }
    #[doc = "Bit 2 - HTISTA6"]
    #[inline(always)]
    pub fn htista6(&mut self) -> HTISTA6_W {
        HTISTA6_W { w: self }
    }
    #[doc = "Bit 3 - TCISTA6"]
    #[inline(always)]
    pub fn tcista6(&mut self) -> TCISTA6_W {
        TCISTA6_W { w: self }
    }
    #[doc = "Bit 4 - TEISTA6"]
    #[inline(always)]
    pub fn teista6(&mut self) -> TEISTA6_W {
        TEISTA6_W { w: self }
    }
    #[doc = "Bit 5 - GEISTA7"]
    #[inline(always)]
    pub fn geista7(&mut self) -> GEISTA7_W {
        GEISTA7_W { w: self }
    }
    #[doc = "Bit 6 - BEISTA7"]
    #[inline(always)]
    pub fn beista7(&mut self) -> BEISTA7_W {
        BEISTA7_W { w: self }
    }
    #[doc = "Bit 7 - HTISTA7"]
    #[inline(always)]
    pub fn htista7(&mut self) -> HTISTA7_W {
        HTISTA7_W { w: self }
    }
    #[doc = "Bit 8 - TCISTA7"]
    #[inline(always)]
    pub fn tcista7(&mut self) -> TCISTA7_W {
        TCISTA7_W { w: self }
    }
    #[doc = "Bit 9 - TEISTA7"]
    #[inline(always)]
    pub fn teista7(&mut self) -> TEISTA7_W {
        TEISTA7_W { w: self }
    }
    #[doc = "Bit 10 - GEISTA8"]
    #[inline(always)]
    pub fn geista8(&mut self) -> GEISTA8_W {
        GEISTA8_W { w: self }
    }
    #[doc = "Bit 11 - BEISTA8"]
    #[inline(always)]
    pub fn beista8(&mut self) -> BEISTA8_W {
        BEISTA8_W { w: self }
    }
    #[doc = "Bit 12 - HTISTA8"]
    #[inline(always)]
    pub fn htista8(&mut self) -> HTISTA8_W {
        HTISTA8_W { w: self }
    }
    #[doc = "Bit 13 - TCISTA8"]
    #[inline(always)]
    pub fn tcista8(&mut self) -> TCISTA8_W {
        TCISTA8_W { w: self }
    }
    #[doc = "Bit 14 - TEISTA8"]
    #[inline(always)]
    pub fn teista8(&mut self) -> TEISTA8_W {
        TEISTA8_W { w: self }
    }
    #[doc = "Bit 15 - GEISTA9"]
    #[inline(always)]
    pub fn geista9(&mut self) -> GEISTA9_W {
        GEISTA9_W { w: self }
    }
    #[doc = "Bit 16 - BEISTA9"]
    #[inline(always)]
    pub fn beista9(&mut self) -> BEISTA9_W {
        BEISTA9_W { w: self }
    }
    #[doc = "Bit 17 - HTISTA9"]
    #[inline(always)]
    pub fn htista9(&mut self) -> HTISTA9_W {
        HTISTA9_W { w: self }
    }
    #[doc = "Bit 18 - TCISTA9"]
    #[inline(always)]
    pub fn tcista9(&mut self) -> TCISTA9_W {
        TCISTA9_W { w: self }
    }
    #[doc = "Bit 19 - TEISTA9"]
    #[inline(always)]
    pub fn teista9(&mut self) -> TEISTA9_W {
        TEISTA9_W { w: self }
    }
    #[doc = "Bit 20 - GEISTA10"]
    #[inline(always)]
    pub fn geista10(&mut self) -> GEISTA10_W {
        GEISTA10_W { w: self }
    }
    #[doc = "Bit 21 - BEISTA10"]
    #[inline(always)]
    pub fn beista10(&mut self) -> BEISTA10_W {
        BEISTA10_W { w: self }
    }
    #[doc = "Bit 22 - HTISTA10"]
    #[inline(always)]
    pub fn htista10(&mut self) -> HTISTA10_W {
        HTISTA10_W { w: self }
    }
    #[doc = "Bit 23 - TCISTA10"]
    #[inline(always)]
    pub fn tcista10(&mut self) -> TCISTA10_W {
        TCISTA10_W { w: self }
    }
    #[doc = "Bit 24 - TEISTA10"]
    #[inline(always)]
    pub fn teista10(&mut self) -> TEISTA10_W {
        TEISTA10_W { w: self }
    }
    #[doc = "Bit 25 - GEISTA11"]
    #[inline(always)]
    pub fn geista11(&mut self) -> GEISTA11_W {
        GEISTA11_W { w: self }
    }
    #[doc = "Bit 26 - BEISTA11"]
    #[inline(always)]
    pub fn beista11(&mut self) -> BEISTA11_W {
        BEISTA11_W { w: self }
    }
    #[doc = "Bit 27 - HTISTA11"]
    #[inline(always)]
    pub fn htista11(&mut self) -> HTISTA11_W {
        HTISTA11_W { w: self }
    }
    #[doc = "Bit 28 - TCISTA11"]
    #[inline(always)]
    pub fn tcista11(&mut self) -> TCISTA11_W {
        TCISTA11_W { w: self }
    }
    #[doc = "Bit 29 - TEISTA11"]
    #[inline(always)]
    pub fn teista11(&mut self) -> TEISTA11_W {
        TEISTA11_W { w: self }
    }
}

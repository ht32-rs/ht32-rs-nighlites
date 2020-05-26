#[doc = "Reader of register PDMA_ISCR1"]
pub type R = crate::R<u32, super::PDMA_ISCR1>;
#[doc = "Writer for register PDMA_ISCR1"]
pub type W = crate::W<u32, super::PDMA_ISCR1>;
#[doc = "Register PDMA_ISCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PDMA_ISCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GEICLR6`"]
pub type GEICLR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEICLR6`"]
pub struct GEICLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> GEICLR6_W<'a> {
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
#[doc = "Reader of field `BEICLR6`"]
pub type BEICLR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEICLR6`"]
pub struct BEICLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> BEICLR6_W<'a> {
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
#[doc = "Reader of field `HTICLR6`"]
pub type HTICLR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTICLR6`"]
pub struct HTICLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> HTICLR6_W<'a> {
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
#[doc = "Reader of field `TCICLR6`"]
pub type TCICLR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCICLR6`"]
pub struct TCICLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> TCICLR6_W<'a> {
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
#[doc = "Reader of field `TEICLR6`"]
pub type TEICLR6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEICLR6`"]
pub struct TEICLR6_W<'a> {
    w: &'a mut W,
}
impl<'a> TEICLR6_W<'a> {
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
#[doc = "Reader of field `GEICLR7`"]
pub type GEICLR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEICLR7`"]
pub struct GEICLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> GEICLR7_W<'a> {
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
#[doc = "Reader of field `BEICLR7`"]
pub type BEICLR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEICLR7`"]
pub struct BEICLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> BEICLR7_W<'a> {
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
#[doc = "Reader of field `HTICLR7`"]
pub type HTICLR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTICLR7`"]
pub struct HTICLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> HTICLR7_W<'a> {
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
#[doc = "Reader of field `TCICLR7`"]
pub type TCICLR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCICLR7`"]
pub struct TCICLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> TCICLR7_W<'a> {
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
#[doc = "Reader of field `TEICLR7`"]
pub type TEICLR7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEICLR7`"]
pub struct TEICLR7_W<'a> {
    w: &'a mut W,
}
impl<'a> TEICLR7_W<'a> {
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
#[doc = "Reader of field `GEICLR8`"]
pub type GEICLR8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEICLR8`"]
pub struct GEICLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> GEICLR8_W<'a> {
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
#[doc = "Reader of field `BEICLR8`"]
pub type BEICLR8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEICLR8`"]
pub struct BEICLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> BEICLR8_W<'a> {
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
#[doc = "Reader of field `HTICLR8`"]
pub type HTICLR8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTICLR8`"]
pub struct HTICLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> HTICLR8_W<'a> {
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
#[doc = "Reader of field `TCICLR8`"]
pub type TCICLR8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCICLR8`"]
pub struct TCICLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> TCICLR8_W<'a> {
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
#[doc = "Reader of field `TEICLR8`"]
pub type TEICLR8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEICLR8`"]
pub struct TEICLR8_W<'a> {
    w: &'a mut W,
}
impl<'a> TEICLR8_W<'a> {
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
#[doc = "Reader of field `GEICLR9`"]
pub type GEICLR9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEICLR9`"]
pub struct GEICLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> GEICLR9_W<'a> {
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
#[doc = "Reader of field `BEICLR9`"]
pub type BEICLR9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEICLR9`"]
pub struct BEICLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> BEICLR9_W<'a> {
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
#[doc = "Reader of field `HTICLR9`"]
pub type HTICLR9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTICLR9`"]
pub struct HTICLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> HTICLR9_W<'a> {
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
#[doc = "Reader of field `TCICLR9`"]
pub type TCICLR9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCICLR9`"]
pub struct TCICLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> TCICLR9_W<'a> {
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
#[doc = "Reader of field `TEICLR9`"]
pub type TEICLR9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEICLR9`"]
pub struct TEICLR9_W<'a> {
    w: &'a mut W,
}
impl<'a> TEICLR9_W<'a> {
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
#[doc = "Reader of field `GEICLR10`"]
pub type GEICLR10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEICLR10`"]
pub struct GEICLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> GEICLR10_W<'a> {
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
#[doc = "Reader of field `BEICLR10`"]
pub type BEICLR10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEICLR10`"]
pub struct BEICLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> BEICLR10_W<'a> {
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
#[doc = "Reader of field `HTICLR10`"]
pub type HTICLR10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTICLR10`"]
pub struct HTICLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> HTICLR10_W<'a> {
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
#[doc = "Reader of field `TCICLR10`"]
pub type TCICLR10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCICLR10`"]
pub struct TCICLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> TCICLR10_W<'a> {
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
#[doc = "Reader of field `TEICLR10`"]
pub type TEICLR10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEICLR10`"]
pub struct TEICLR10_W<'a> {
    w: &'a mut W,
}
impl<'a> TEICLR10_W<'a> {
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
#[doc = "Reader of field `GEICLR11`"]
pub type GEICLR11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEICLR11`"]
pub struct GEICLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> GEICLR11_W<'a> {
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
#[doc = "Reader of field `BEICLR11`"]
pub type BEICLR11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEICLR11`"]
pub struct BEICLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> BEICLR11_W<'a> {
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
#[doc = "Reader of field `HTICLR11`"]
pub type HTICLR11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTICLR11`"]
pub struct HTICLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> HTICLR11_W<'a> {
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
#[doc = "Reader of field `TCICLR11`"]
pub type TCICLR11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCICLR11`"]
pub struct TCICLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> TCICLR11_W<'a> {
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
#[doc = "Reader of field `TEICLR11`"]
pub type TEICLR11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEICLR11`"]
pub struct TEICLR11_W<'a> {
    w: &'a mut W,
}
impl<'a> TEICLR11_W<'a> {
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
    #[doc = "Bit 0 - GEICLR6"]
    #[inline(always)]
    pub fn geiclr6(&self) -> GEICLR6_R {
        GEICLR6_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BEICLR6"]
    #[inline(always)]
    pub fn beiclr6(&self) -> BEICLR6_R {
        BEICLR6_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HTICLR6"]
    #[inline(always)]
    pub fn hticlr6(&self) -> HTICLR6_R {
        HTICLR6_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TCICLR6"]
    #[inline(always)]
    pub fn tciclr6(&self) -> TCICLR6_R {
        TCICLR6_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TEICLR6"]
    #[inline(always)]
    pub fn teiclr6(&self) -> TEICLR6_R {
        TEICLR6_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GEICLR7"]
    #[inline(always)]
    pub fn geiclr7(&self) -> GEICLR7_R {
        GEICLR7_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BEICLR7"]
    #[inline(always)]
    pub fn beiclr7(&self) -> BEICLR7_R {
        BEICLR7_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - HTICLR7"]
    #[inline(always)]
    pub fn hticlr7(&self) -> HTICLR7_R {
        HTICLR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TCICLR7"]
    #[inline(always)]
    pub fn tciclr7(&self) -> TCICLR7_R {
        TCICLR7_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TEICLR7"]
    #[inline(always)]
    pub fn teiclr7(&self) -> TEICLR7_R {
        TEICLR7_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GEICLR8"]
    #[inline(always)]
    pub fn geiclr8(&self) -> GEICLR8_R {
        GEICLR8_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BEICLR8"]
    #[inline(always)]
    pub fn beiclr8(&self) -> BEICLR8_R {
        BEICLR8_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - HTICLR8"]
    #[inline(always)]
    pub fn hticlr8(&self) -> HTICLR8_R {
        HTICLR8_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TCICLR8"]
    #[inline(always)]
    pub fn tciclr8(&self) -> TCICLR8_R {
        TCICLR8_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TEICLR8"]
    #[inline(always)]
    pub fn teiclr8(&self) -> TEICLR8_R {
        TEICLR8_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GEICLR9"]
    #[inline(always)]
    pub fn geiclr9(&self) -> GEICLR9_R {
        GEICLR9_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BEICLR9"]
    #[inline(always)]
    pub fn beiclr9(&self) -> BEICLR9_R {
        BEICLR9_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HTICLR9"]
    #[inline(always)]
    pub fn hticlr9(&self) -> HTICLR9_R {
        HTICLR9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TCICLR9"]
    #[inline(always)]
    pub fn tciclr9(&self) -> TCICLR9_R {
        TCICLR9_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TEICLR9"]
    #[inline(always)]
    pub fn teiclr9(&self) -> TEICLR9_R {
        TEICLR9_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - GEICLR10"]
    #[inline(always)]
    pub fn geiclr10(&self) -> GEICLR10_R {
        GEICLR10_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - BEICLR10"]
    #[inline(always)]
    pub fn beiclr10(&self) -> BEICLR10_R {
        BEICLR10_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - HTICLR10"]
    #[inline(always)]
    pub fn hticlr10(&self) -> HTICLR10_R {
        HTICLR10_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TCICLR10"]
    #[inline(always)]
    pub fn tciclr10(&self) -> TCICLR10_R {
        TCICLR10_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TEICLR10"]
    #[inline(always)]
    pub fn teiclr10(&self) -> TEICLR10_R {
        TEICLR10_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - GEICLR11"]
    #[inline(always)]
    pub fn geiclr11(&self) -> GEICLR11_R {
        GEICLR11_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - BEICLR11"]
    #[inline(always)]
    pub fn beiclr11(&self) -> BEICLR11_R {
        BEICLR11_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - HTICLR11"]
    #[inline(always)]
    pub fn hticlr11(&self) -> HTICLR11_R {
        HTICLR11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - TCICLR11"]
    #[inline(always)]
    pub fn tciclr11(&self) -> TCICLR11_R {
        TCICLR11_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - TEICLR11"]
    #[inline(always)]
    pub fn teiclr11(&self) -> TEICLR11_R {
        TEICLR11_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GEICLR6"]
    #[inline(always)]
    pub fn geiclr6(&mut self) -> GEICLR6_W {
        GEICLR6_W { w: self }
    }
    #[doc = "Bit 1 - BEICLR6"]
    #[inline(always)]
    pub fn beiclr6(&mut self) -> BEICLR6_W {
        BEICLR6_W { w: self }
    }
    #[doc = "Bit 2 - HTICLR6"]
    #[inline(always)]
    pub fn hticlr6(&mut self) -> HTICLR6_W {
        HTICLR6_W { w: self }
    }
    #[doc = "Bit 3 - TCICLR6"]
    #[inline(always)]
    pub fn tciclr6(&mut self) -> TCICLR6_W {
        TCICLR6_W { w: self }
    }
    #[doc = "Bit 4 - TEICLR6"]
    #[inline(always)]
    pub fn teiclr6(&mut self) -> TEICLR6_W {
        TEICLR6_W { w: self }
    }
    #[doc = "Bit 5 - GEICLR7"]
    #[inline(always)]
    pub fn geiclr7(&mut self) -> GEICLR7_W {
        GEICLR7_W { w: self }
    }
    #[doc = "Bit 6 - BEICLR7"]
    #[inline(always)]
    pub fn beiclr7(&mut self) -> BEICLR7_W {
        BEICLR7_W { w: self }
    }
    #[doc = "Bit 7 - HTICLR7"]
    #[inline(always)]
    pub fn hticlr7(&mut self) -> HTICLR7_W {
        HTICLR7_W { w: self }
    }
    #[doc = "Bit 8 - TCICLR7"]
    #[inline(always)]
    pub fn tciclr7(&mut self) -> TCICLR7_W {
        TCICLR7_W { w: self }
    }
    #[doc = "Bit 9 - TEICLR7"]
    #[inline(always)]
    pub fn teiclr7(&mut self) -> TEICLR7_W {
        TEICLR7_W { w: self }
    }
    #[doc = "Bit 10 - GEICLR8"]
    #[inline(always)]
    pub fn geiclr8(&mut self) -> GEICLR8_W {
        GEICLR8_W { w: self }
    }
    #[doc = "Bit 11 - BEICLR8"]
    #[inline(always)]
    pub fn beiclr8(&mut self) -> BEICLR8_W {
        BEICLR8_W { w: self }
    }
    #[doc = "Bit 12 - HTICLR8"]
    #[inline(always)]
    pub fn hticlr8(&mut self) -> HTICLR8_W {
        HTICLR8_W { w: self }
    }
    #[doc = "Bit 13 - TCICLR8"]
    #[inline(always)]
    pub fn tciclr8(&mut self) -> TCICLR8_W {
        TCICLR8_W { w: self }
    }
    #[doc = "Bit 14 - TEICLR8"]
    #[inline(always)]
    pub fn teiclr8(&mut self) -> TEICLR8_W {
        TEICLR8_W { w: self }
    }
    #[doc = "Bit 15 - GEICLR9"]
    #[inline(always)]
    pub fn geiclr9(&mut self) -> GEICLR9_W {
        GEICLR9_W { w: self }
    }
    #[doc = "Bit 16 - BEICLR9"]
    #[inline(always)]
    pub fn beiclr9(&mut self) -> BEICLR9_W {
        BEICLR9_W { w: self }
    }
    #[doc = "Bit 17 - HTICLR9"]
    #[inline(always)]
    pub fn hticlr9(&mut self) -> HTICLR9_W {
        HTICLR9_W { w: self }
    }
    #[doc = "Bit 18 - TCICLR9"]
    #[inline(always)]
    pub fn tciclr9(&mut self) -> TCICLR9_W {
        TCICLR9_W { w: self }
    }
    #[doc = "Bit 19 - TEICLR9"]
    #[inline(always)]
    pub fn teiclr9(&mut self) -> TEICLR9_W {
        TEICLR9_W { w: self }
    }
    #[doc = "Bit 20 - GEICLR10"]
    #[inline(always)]
    pub fn geiclr10(&mut self) -> GEICLR10_W {
        GEICLR10_W { w: self }
    }
    #[doc = "Bit 21 - BEICLR10"]
    #[inline(always)]
    pub fn beiclr10(&mut self) -> BEICLR10_W {
        BEICLR10_W { w: self }
    }
    #[doc = "Bit 22 - HTICLR10"]
    #[inline(always)]
    pub fn hticlr10(&mut self) -> HTICLR10_W {
        HTICLR10_W { w: self }
    }
    #[doc = "Bit 23 - TCICLR10"]
    #[inline(always)]
    pub fn tciclr10(&mut self) -> TCICLR10_W {
        TCICLR10_W { w: self }
    }
    #[doc = "Bit 24 - TEICLR10"]
    #[inline(always)]
    pub fn teiclr10(&mut self) -> TEICLR10_W {
        TEICLR10_W { w: self }
    }
    #[doc = "Bit 25 - GEICLR11"]
    #[inline(always)]
    pub fn geiclr11(&mut self) -> GEICLR11_W {
        GEICLR11_W { w: self }
    }
    #[doc = "Bit 26 - BEICLR11"]
    #[inline(always)]
    pub fn beiclr11(&mut self) -> BEICLR11_W {
        BEICLR11_W { w: self }
    }
    #[doc = "Bit 27 - HTICLR11"]
    #[inline(always)]
    pub fn hticlr11(&mut self) -> HTICLR11_W {
        HTICLR11_W { w: self }
    }
    #[doc = "Bit 28 - TCICLR11"]
    #[inline(always)]
    pub fn tciclr11(&mut self) -> TCICLR11_W {
        TCICLR11_W { w: self }
    }
    #[doc = "Bit 29 - TEICLR11"]
    #[inline(always)]
    pub fn teiclr11(&mut self) -> TEICLR11_W {
        TEICLR11_W { w: self }
    }
}

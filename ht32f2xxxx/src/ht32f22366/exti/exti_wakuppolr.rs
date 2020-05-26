#[doc = "Reader of register EXTI_WAKUPPOLR"]
pub type R = crate::R<u32, super::EXTI_WAKUPPOLR>;
#[doc = "Writer for register EXTI_WAKUPPOLR"]
pub type W = crate::W<u32, super::EXTI_WAKUPPOLR>;
#[doc = "Register EXTI_WAKUPPOLR `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTI_WAKUPPOLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI0POL`"]
pub type EXTI0POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI0POL`"]
pub struct EXTI0POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0POL_W<'a> {
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
#[doc = "Reader of field `EXTI1POL`"]
pub type EXTI1POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI1POL`"]
pub struct EXTI1POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI1POL_W<'a> {
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
#[doc = "Reader of field `EXTI2POL`"]
pub type EXTI2POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI2POL`"]
pub struct EXTI2POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI2POL_W<'a> {
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
#[doc = "Reader of field `EXTI3POL`"]
pub type EXTI3POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI3POL`"]
pub struct EXTI3POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI3POL_W<'a> {
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
#[doc = "Reader of field `EXTI4POL`"]
pub type EXTI4POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI4POL`"]
pub struct EXTI4POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI4POL_W<'a> {
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
#[doc = "Reader of field `EXTI5POL`"]
pub type EXTI5POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI5POL`"]
pub struct EXTI5POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI5POL_W<'a> {
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
#[doc = "Reader of field `EXTI6POL`"]
pub type EXTI6POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI6POL`"]
pub struct EXTI6POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI6POL_W<'a> {
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
#[doc = "Reader of field `EXTI7POL`"]
pub type EXTI7POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI7POL`"]
pub struct EXTI7POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI7POL_W<'a> {
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
#[doc = "Reader of field `EXTI8POL`"]
pub type EXTI8POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI8POL`"]
pub struct EXTI8POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8POL_W<'a> {
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
#[doc = "Reader of field `EXTI9POL`"]
pub type EXTI9POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI9POL`"]
pub struct EXTI9POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9POL_W<'a> {
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
#[doc = "Reader of field `EXTI10POL`"]
pub type EXTI10POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI10POL`"]
pub struct EXTI10POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10POL_W<'a> {
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
#[doc = "Reader of field `EXTI11POL`"]
pub type EXTI11POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI11POL`"]
pub struct EXTI11POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11POL_W<'a> {
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
#[doc = "Reader of field `EXTI12POL`"]
pub type EXTI12POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI12POL`"]
pub struct EXTI12POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12POL_W<'a> {
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
#[doc = "Reader of field `EXTI13POL`"]
pub type EXTI13POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI13POL`"]
pub struct EXTI13POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13POL_W<'a> {
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
#[doc = "Reader of field `EXTI14POL`"]
pub type EXTI14POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI14POL`"]
pub struct EXTI14POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14POL_W<'a> {
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
#[doc = "Reader of field `EXTI15POL`"]
pub type EXTI15POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXTI15POL`"]
pub struct EXTI15POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15POL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - EXTI0POL"]
    #[inline(always)]
    pub fn exti0pol(&self) -> EXTI0POL_R {
        EXTI0POL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EXTI1POL"]
    #[inline(always)]
    pub fn exti1pol(&self) -> EXTI1POL_R {
        EXTI1POL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EXTI2POL"]
    #[inline(always)]
    pub fn exti2pol(&self) -> EXTI2POL_R {
        EXTI2POL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EXTI3POL"]
    #[inline(always)]
    pub fn exti3pol(&self) -> EXTI3POL_R {
        EXTI3POL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EXTI4POL"]
    #[inline(always)]
    pub fn exti4pol(&self) -> EXTI4POL_R {
        EXTI4POL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EXTI5POL"]
    #[inline(always)]
    pub fn exti5pol(&self) -> EXTI5POL_R {
        EXTI5POL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EXTI6POL"]
    #[inline(always)]
    pub fn exti6pol(&self) -> EXTI6POL_R {
        EXTI6POL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EXTI7POL"]
    #[inline(always)]
    pub fn exti7pol(&self) -> EXTI7POL_R {
        EXTI7POL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EXTI8POL"]
    #[inline(always)]
    pub fn exti8pol(&self) -> EXTI8POL_R {
        EXTI8POL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EXTI9POL"]
    #[inline(always)]
    pub fn exti9pol(&self) -> EXTI9POL_R {
        EXTI9POL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EXTI10POL"]
    #[inline(always)]
    pub fn exti10pol(&self) -> EXTI10POL_R {
        EXTI10POL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - EXTI11POL"]
    #[inline(always)]
    pub fn exti11pol(&self) -> EXTI11POL_R {
        EXTI11POL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EXTI12POL"]
    #[inline(always)]
    pub fn exti12pol(&self) -> EXTI12POL_R {
        EXTI12POL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EXTI13POL"]
    #[inline(always)]
    pub fn exti13pol(&self) -> EXTI13POL_R {
        EXTI13POL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EXTI14POL"]
    #[inline(always)]
    pub fn exti14pol(&self) -> EXTI14POL_R {
        EXTI14POL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EXTI15POL"]
    #[inline(always)]
    pub fn exti15pol(&self) -> EXTI15POL_R {
        EXTI15POL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXTI0POL"]
    #[inline(always)]
    pub fn exti0pol(&mut self) -> EXTI0POL_W {
        EXTI0POL_W { w: self }
    }
    #[doc = "Bit 1 - EXTI1POL"]
    #[inline(always)]
    pub fn exti1pol(&mut self) -> EXTI1POL_W {
        EXTI1POL_W { w: self }
    }
    #[doc = "Bit 2 - EXTI2POL"]
    #[inline(always)]
    pub fn exti2pol(&mut self) -> EXTI2POL_W {
        EXTI2POL_W { w: self }
    }
    #[doc = "Bit 3 - EXTI3POL"]
    #[inline(always)]
    pub fn exti3pol(&mut self) -> EXTI3POL_W {
        EXTI3POL_W { w: self }
    }
    #[doc = "Bit 4 - EXTI4POL"]
    #[inline(always)]
    pub fn exti4pol(&mut self) -> EXTI4POL_W {
        EXTI4POL_W { w: self }
    }
    #[doc = "Bit 5 - EXTI5POL"]
    #[inline(always)]
    pub fn exti5pol(&mut self) -> EXTI5POL_W {
        EXTI5POL_W { w: self }
    }
    #[doc = "Bit 6 - EXTI6POL"]
    #[inline(always)]
    pub fn exti6pol(&mut self) -> EXTI6POL_W {
        EXTI6POL_W { w: self }
    }
    #[doc = "Bit 7 - EXTI7POL"]
    #[inline(always)]
    pub fn exti7pol(&mut self) -> EXTI7POL_W {
        EXTI7POL_W { w: self }
    }
    #[doc = "Bit 8 - EXTI8POL"]
    #[inline(always)]
    pub fn exti8pol(&mut self) -> EXTI8POL_W {
        EXTI8POL_W { w: self }
    }
    #[doc = "Bit 9 - EXTI9POL"]
    #[inline(always)]
    pub fn exti9pol(&mut self) -> EXTI9POL_W {
        EXTI9POL_W { w: self }
    }
    #[doc = "Bit 10 - EXTI10POL"]
    #[inline(always)]
    pub fn exti10pol(&mut self) -> EXTI10POL_W {
        EXTI10POL_W { w: self }
    }
    #[doc = "Bit 11 - EXTI11POL"]
    #[inline(always)]
    pub fn exti11pol(&mut self) -> EXTI11POL_W {
        EXTI11POL_W { w: self }
    }
    #[doc = "Bit 12 - EXTI12POL"]
    #[inline(always)]
    pub fn exti12pol(&mut self) -> EXTI12POL_W {
        EXTI12POL_W { w: self }
    }
    #[doc = "Bit 13 - EXTI13POL"]
    #[inline(always)]
    pub fn exti13pol(&mut self) -> EXTI13POL_W {
        EXTI13POL_W { w: self }
    }
    #[doc = "Bit 14 - EXTI14POL"]
    #[inline(always)]
    pub fn exti14pol(&mut self) -> EXTI14POL_W {
        EXTI14POL_W { w: self }
    }
    #[doc = "Bit 15 - EXTI15POL"]
    #[inline(always)]
    pub fn exti15pol(&mut self) -> EXTI15POL_W {
        EXTI15POL_W { w: self }
    }
}

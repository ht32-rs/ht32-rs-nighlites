#[doc = "Reader of register CSIF_SR"]
pub type R = crate::R<u32, super::CSIF_SR>;
#[doc = "Writer for register CSIF_SR"]
pub type W = crate::W<u32, super::CSIF_SR>;
#[doc = "Register CSIF_SR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSIF_SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOF_FLG`"]
pub type SOF_FLG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOF_FLG`"]
pub struct SOF_FLG_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_FLG_W<'a> {
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
#[doc = "Reader of field `EOF_FLG`"]
pub type EOF_FLG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOF_FLG`"]
pub struct EOF_FLG_W<'a> {
    w: &'a mut W,
}
impl<'a> EOF_FLG_W<'a> {
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
#[doc = "Reader of field `CAP_STA`"]
pub type CAP_STA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP_STA`"]
pub struct CAP_STA_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_STA_W<'a> {
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
#[doc = "Reader of field `CAP_STS`"]
pub type CAP_STS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP_STS`"]
pub struct CAP_STS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_STS_W<'a> {
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
#[doc = "Reader of field `BAD_FRM`"]
pub type BAD_FRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BAD_FRM`"]
pub struct BAD_FRM_W<'a> {
    w: &'a mut W,
}
impl<'a> BAD_FRM_W<'a> {
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
#[doc = "Reader of field `FIFO_OVR`"]
pub type FIFO_OVR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_OVR`"]
pub struct FIFO_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_OVR_W<'a> {
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
#[doc = "Reader of field `FIFO_EMP`"]
pub type FIFO_EMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_EMP`"]
pub struct FIFO_EMP_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_EMP_W<'a> {
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
#[doc = "Reader of field `FIFO_FUL`"]
pub type FIFO_FUL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFO_FUL`"]
pub struct FIFO_FUL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_FUL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SOF_FLG"]
    #[inline(always)]
    pub fn sof_flg(&self) -> SOF_FLG_R {
        SOF_FLG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EOF_FLG"]
    #[inline(always)]
    pub fn eof_flg(&self) -> EOF_FLG_R {
        EOF_FLG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CAP_STA"]
    #[inline(always)]
    pub fn cap_sta(&self) -> CAP_STA_R {
        CAP_STA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CAP_STS"]
    #[inline(always)]
    pub fn cap_sts(&self) -> CAP_STS_R {
        CAP_STS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BAD_FRM"]
    #[inline(always)]
    pub fn bad_frm(&self) -> BAD_FRM_R {
        BAD_FRM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FIFO_OVR"]
    #[inline(always)]
    pub fn fifo_ovr(&self) -> FIFO_OVR_R {
        FIFO_OVR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FIFO_EMP"]
    #[inline(always)]
    pub fn fifo_emp(&self) -> FIFO_EMP_R {
        FIFO_EMP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FIFO_FUL"]
    #[inline(always)]
    pub fn fifo_ful(&self) -> FIFO_FUL_R {
        FIFO_FUL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SOF_FLG"]
    #[inline(always)]
    pub fn sof_flg(&mut self) -> SOF_FLG_W {
        SOF_FLG_W { w: self }
    }
    #[doc = "Bit 1 - EOF_FLG"]
    #[inline(always)]
    pub fn eof_flg(&mut self) -> EOF_FLG_W {
        EOF_FLG_W { w: self }
    }
    #[doc = "Bit 2 - CAP_STA"]
    #[inline(always)]
    pub fn cap_sta(&mut self) -> CAP_STA_W {
        CAP_STA_W { w: self }
    }
    #[doc = "Bit 3 - CAP_STS"]
    #[inline(always)]
    pub fn cap_sts(&mut self) -> CAP_STS_W {
        CAP_STS_W { w: self }
    }
    #[doc = "Bit 4 - BAD_FRM"]
    #[inline(always)]
    pub fn bad_frm(&mut self) -> BAD_FRM_W {
        BAD_FRM_W { w: self }
    }
    #[doc = "Bit 8 - FIFO_OVR"]
    #[inline(always)]
    pub fn fifo_ovr(&mut self) -> FIFO_OVR_W {
        FIFO_OVR_W { w: self }
    }
    #[doc = "Bit 9 - FIFO_EMP"]
    #[inline(always)]
    pub fn fifo_emp(&mut self) -> FIFO_EMP_W {
        FIFO_EMP_W { w: self }
    }
    #[doc = "Bit 10 - FIFO_FUL"]
    #[inline(always)]
    pub fn fifo_ful(&mut self) -> FIFO_FUL_W {
        FIFO_FUL_W { w: self }
    }
}

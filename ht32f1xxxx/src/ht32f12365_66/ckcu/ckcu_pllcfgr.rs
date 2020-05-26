#[doc = "Reader of register CKCU_PLLCFGR"]
pub type R = crate::R<u32, super::CKCU_PLLCFGR>;
#[doc = "Writer for register CKCU_PLLCFGR"]
pub type W = crate::W<u32, super::CKCU_PLLCFGR>;
#[doc = "Register CKCU_PLLCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::CKCU_PLLCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `POTD`"]
pub type POTD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POTD`"]
pub struct POTD_W<'a> {
    w: &'a mut W,
}
impl<'a> POTD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `PFBD`"]
pub type PFBD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PFBD`"]
pub struct PFBD_W<'a> {
    w: &'a mut W,
}
impl<'a> PFBD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 23)) | (((value as u32) & 0x3f) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 21:22 - POTD"]
    #[inline(always)]
    pub fn potd(&self) -> POTD_R {
        POTD_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 23:28 - PFBD"]
    #[inline(always)]
    pub fn pfbd(&self) -> PFBD_R {
        PFBD_R::new(((self.bits >> 23) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 21:22 - POTD"]
    #[inline(always)]
    pub fn potd(&mut self) -> POTD_W {
        POTD_W { w: self }
    }
    #[doc = "Bits 23:28 - PFBD"]
    #[inline(always)]
    pub fn pfbd(&mut self) -> PFBD_W {
        PFBD_W { w: self }
    }
}

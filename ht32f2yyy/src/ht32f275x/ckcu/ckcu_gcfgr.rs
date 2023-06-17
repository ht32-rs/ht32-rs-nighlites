#[doc = "Register `CKCU_GCFGR` reader"]
pub struct R(crate::R<CKCU_GCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_GCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_GCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_GCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_GCFGR` writer"]
pub struct W(crate::W<CKCU_GCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_GCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CKCU_GCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_GCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKOUTSRC` reader - CKOUTSRC"]
pub type CKOUTSRC_R = crate::FieldReader;
#[doc = "Field `CKOUTSRC` writer - CKOUTSRC"]
pub type CKOUTSRC_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_GCFGR_SPEC, 3, O>;
#[doc = "Field `WDTSRC` reader - WDTSRC"]
pub type WDTSRC_R = crate::BitReader;
#[doc = "Field `WDTSRC` writer - WDTSRC"]
pub type WDTSRC_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_GCFGR_SPEC, O>;
#[doc = "Field `PLLSRC` reader - PLLSRC"]
pub type PLLSRC_R = crate::BitReader;
#[doc = "Field `PLLSRC` writer - PLLSRC"]
pub type PLLSRC_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_GCFGR_SPEC, O>;
#[doc = "Field `URPRE` reader - URPRE"]
pub type URPRE_R = crate::FieldReader;
#[doc = "Field `URPRE` writer - URPRE"]
pub type URPRE_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_GCFGR_SPEC, 2, O>;
#[doc = "Field `USBPRE` reader - USBPRE"]
pub type USBPRE_R = crate::FieldReader;
#[doc = "Field `USBPRE` writer - USBPRE"]
pub type USBPRE_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_GCFGR_SPEC, 2, O>;
#[doc = "Field `CSIFMPRE` reader - CSIFMPRE"]
pub type CSIFMPRE_R = crate::FieldReader;
#[doc = "Field `CSIFMPRE` writer - CSIFMPRE"]
pub type CSIFMPRE_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_GCFGR_SPEC, 5, O>;
#[doc = "Field `LPMOD` reader - LPMOD"]
pub type LPMOD_R = crate::FieldReader;
#[doc = "Field `LPMOD` writer - LPMOD"]
pub type LPMOD_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_GCFGR_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:2 - CKOUTSRC"]
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CKOUTSRC_R {
        CKOUTSRC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - WDTSRC"]
    #[inline(always)]
    pub fn wdtsrc(&self) -> WDTSRC_R {
        WDTSRC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - PLLSRC"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 20:21 - URPRE"]
    #[inline(always)]
    pub fn urpre(&self) -> URPRE_R {
        URPRE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - USBPRE"]
    #[inline(always)]
    pub fn usbpre(&self) -> USBPRE_R {
        USBPRE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:28 - CSIFMPRE"]
    #[inline(always)]
    pub fn csifmpre(&self) -> CSIFMPRE_R {
        CSIFMPRE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - LPMOD"]
    #[inline(always)]
    pub fn lpmod(&self) -> LPMOD_R {
        LPMOD_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CKOUTSRC"]
    #[inline(always)]
    #[must_use]
    pub fn ckoutsrc(&mut self) -> CKOUTSRC_W<0> {
        CKOUTSRC_W::new(self)
    }
    #[doc = "Bit 3 - WDTSRC"]
    #[inline(always)]
    #[must_use]
    pub fn wdtsrc(&mut self) -> WDTSRC_W<3> {
        WDTSRC_W::new(self)
    }
    #[doc = "Bit 8 - PLLSRC"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<8> {
        PLLSRC_W::new(self)
    }
    #[doc = "Bits 20:21 - URPRE"]
    #[inline(always)]
    #[must_use]
    pub fn urpre(&mut self) -> URPRE_W<20> {
        URPRE_W::new(self)
    }
    #[doc = "Bits 22:23 - USBPRE"]
    #[inline(always)]
    #[must_use]
    pub fn usbpre(&mut self) -> USBPRE_W<22> {
        USBPRE_W::new(self)
    }
    #[doc = "Bits 24:28 - CSIFMPRE"]
    #[inline(always)]
    #[must_use]
    pub fn csifmpre(&mut self) -> CSIFMPRE_W<24> {
        CSIFMPRE_W::new(self)
    }
    #[doc = "Bits 29:31 - LPMOD"]
    #[inline(always)]
    #[must_use]
    pub fn lpmod(&mut self) -> LPMOD_W<29> {
        LPMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_GCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_gcfgr](index.html) module"]
pub struct CKCU_GCFGR_SPEC;
impl crate::RegisterSpec for CKCU_GCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_gcfgr::R](R) reader structure"]
impl crate::Readable for CKCU_GCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_gcfgr::W](W) writer structure"]
impl crate::Writable for CKCU_GCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_GCFGR to value 0"]
impl crate::Resettable for CKCU_GCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

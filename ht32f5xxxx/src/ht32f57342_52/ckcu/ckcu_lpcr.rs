#[doc = "Register `CKCU_LPCR` reader"]
pub struct R(crate::R<CKCU_LPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_LPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_LPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_LPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_LPCR` writer"]
pub struct W(crate::W<CKCU_LPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_LPCR_SPEC>;
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
impl From<crate::W<CKCU_LPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_LPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBSLEEP` reader - USBSLEEP"]
pub type USBSLEEP_R = crate::BitReader;
#[doc = "Field `USBSLEEP` writer - USBSLEEP"]
pub type USBSLEEP_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_LPCR_SPEC, O>;
impl R {
    #[doc = "Bit 8 - USBSLEEP"]
    #[inline(always)]
    pub fn usbsleep(&self) -> USBSLEEP_R {
        USBSLEEP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - USBSLEEP"]
    #[inline(always)]
    #[must_use]
    pub fn usbsleep(&mut self) -> USBSLEEP_W<8> {
        USBSLEEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_LPCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_lpcr](index.html) module"]
pub struct CKCU_LPCR_SPEC;
impl crate::RegisterSpec for CKCU_LPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_lpcr::R](R) reader structure"]
impl crate::Readable for CKCU_LPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_lpcr::W](W) writer structure"]
impl crate::Writable for CKCU_LPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_LPCR to value 0"]
impl crate::Resettable for CKCU_LPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

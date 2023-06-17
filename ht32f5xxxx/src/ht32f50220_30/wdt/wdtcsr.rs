#[doc = "Register `WDTCSR` reader"]
pub struct R(crate::R<WDTCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCSR` writer"]
pub struct W(crate::W<WDTCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCSR_SPEC>;
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
impl From<crate::W<WDTCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTSRC` reader - WDTSRC"]
pub type WDTSRC_R = crate::BitReader;
#[doc = "Field `WDTSRC` writer - WDTSRC"]
pub type WDTSRC_W<'a, const O: u8> = crate::BitWriter<'a, WDTCSR_SPEC, O>;
#[doc = "Field `WDTLOCK` reader - WDTLOCK"]
pub type WDTLOCK_R = crate::BitReader;
#[doc = "Field `WDTLOCK` writer - WDTLOCK"]
pub type WDTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, WDTCSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - WDTSRC"]
    #[inline(always)]
    pub fn wdtsrc(&self) -> WDTSRC_R {
        WDTSRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - WDTLOCK"]
    #[inline(always)]
    pub fn wdtlock(&self) -> WDTLOCK_R {
        WDTLOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDTSRC"]
    #[inline(always)]
    #[must_use]
    pub fn wdtsrc(&mut self) -> WDTSRC_W<0> {
        WDTSRC_W::new(self)
    }
    #[doc = "Bit 4 - WDTLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn wdtlock(&mut self) -> WDTLOCK_W<4> {
        WDTLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDTCSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtcsr](index.html) module"]
pub struct WDTCSR_SPEC;
impl crate::RegisterSpec for WDTCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtcsr::R](R) reader structure"]
impl crate::Readable for WDTCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtcsr::W](W) writer structure"]
impl crate::Writable for WDTCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCSR to value 0"]
impl crate::Resettable for WDTCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

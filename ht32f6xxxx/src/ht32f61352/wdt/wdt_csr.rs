#[doc = "Register `WDT_CSR` reader"]
pub struct R(crate::R<WDT_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_CSR` writer"]
pub struct W(crate::W<WDT_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_CSR_SPEC>;
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
impl From<crate::W<WDT_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDT_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTSRC` reader - WDTSRC"]
pub type WDTSRC_R = crate::BitReader;
#[doc = "Field `WDTSRC` writer - WDTSRC"]
pub type WDTSRC_W<'a, const O: u8> = crate::BitWriter<'a, WDT_CSR_SPEC, O>;
#[doc = "Field `WDTLOCK` reader - WDTLOCK"]
pub type WDTLOCK_R = crate::BitReader;
#[doc = "Field `WDTLOCK` writer - WDTLOCK"]
pub type WDTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, WDT_CSR_SPEC, O>;
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
#[doc = "WDT_CSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_csr](index.html) module"]
pub struct WDT_CSR_SPEC;
impl crate::RegisterSpec for WDT_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_csr::R](R) reader structure"]
impl crate::Readable for WDT_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_csr::W](W) writer structure"]
impl crate::Writable for WDT_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDT_CSR to value 0"]
impl crate::Resettable for WDT_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

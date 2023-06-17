#[doc = "Register `WDT_PR` reader"]
pub struct R(crate::R<WDT_PR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_PR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_PR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_PR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_PR` writer"]
pub struct W(crate::W<WDT_PR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_PR_SPEC>;
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
impl From<crate::W<WDT_PR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDT_PR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROTECT` reader - PROTECT"]
pub type PROTECT_R = crate::FieldReader<u16>;
#[doc = "Field `PROTECT` writer - PROTECT"]
pub type PROTECT_W<'a, const O: u8> = crate::FieldWriter<'a, WDT_PR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PROTECT"]
    #[inline(always)]
    pub fn protect(&self) -> PROTECT_R {
        PROTECT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PROTECT"]
    #[inline(always)]
    #[must_use]
    pub fn protect(&mut self) -> PROTECT_W<0> {
        PROTECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT_PR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_pr](index.html) module"]
pub struct WDT_PR_SPEC;
impl crate::RegisterSpec for WDT_PR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_pr::R](R) reader structure"]
impl crate::Readable for WDT_PR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_pr::W](W) writer structure"]
impl crate::Writable for WDT_PR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDT_PR to value 0"]
impl crate::Resettable for WDT_PR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

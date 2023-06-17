#[doc = "Register `WDTPR` reader"]
pub struct R(crate::R<WDTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTPR` writer"]
pub struct W(crate::W<WDTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTPR_SPEC>;
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
impl From<crate::W<WDTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROTECT` reader - PROTECT"]
pub type PROTECT_R = crate::FieldReader<u16>;
#[doc = "Field `PROTECT` writer - PROTECT"]
pub type PROTECT_W<'a, const O: u8> = crate::FieldWriter<'a, WDTPR_SPEC, 16, O, u16>;
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
#[doc = "WDTPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtpr](index.html) module"]
pub struct WDTPR_SPEC;
impl crate::RegisterSpec for WDTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtpr::R](R) reader structure"]
impl crate::Readable for WDTPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtpr::W](W) writer structure"]
impl crate::Writable for WDTPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTPR to value 0"]
impl crate::Resettable for WDTPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

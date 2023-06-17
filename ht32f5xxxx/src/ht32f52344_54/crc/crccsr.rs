#[doc = "Register `CRCCSR` reader"]
pub struct R(crate::R<CRCCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCCSR` writer"]
pub struct W(crate::W<CRCCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCCSR_SPEC>;
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
impl From<crate::W<CRCCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHKSUM` reader - CHKSUM"]
pub type CHKSUM_R = crate::FieldReader<u32>;
#[doc = "Field `CHKSUM` writer - CHKSUM"]
pub type CHKSUM_W<'a, const O: u8> = crate::FieldWriter<'a, CRCCSR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CHKSUM"]
    #[inline(always)]
    pub fn chksum(&self) -> CHKSUM_R {
        CHKSUM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CHKSUM"]
    #[inline(always)]
    #[must_use]
    pub fn chksum(&mut self) -> CHKSUM_W<0> {
        CHKSUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRCCSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crccsr](index.html) module"]
pub struct CRCCSR_SPEC;
impl crate::RegisterSpec for CRCCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crccsr::R](R) reader structure"]
impl crate::Readable for CRCCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crccsr::W](W) writer structure"]
impl crate::Writable for CRCCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCCSR to value 0"]
impl crate::Resettable for CRCCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

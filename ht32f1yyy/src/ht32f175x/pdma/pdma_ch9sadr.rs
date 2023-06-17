#[doc = "Register `PDMA_CH9SADR` reader"]
pub struct R(crate::R<PDMA_CH9SADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_CH9SADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_CH9SADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_CH9SADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_CH9SADR` writer"]
pub struct W(crate::W<PDMA_CH9SADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_CH9SADR_SPEC>;
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
impl From<crate::W<PDMA_CH9SADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_CH9SADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADR` reader - SADR"]
pub type SADR_R = crate::FieldReader<u32>;
#[doc = "Field `SADR` writer - SADR"]
pub type SADR_W<'a, const O: u8> = crate::FieldWriter<'a, PDMA_CH9SADR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - SADR"]
    #[inline(always)]
    pub fn sadr(&self) -> SADR_R {
        SADR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SADR"]
    #[inline(always)]
    #[must_use]
    pub fn sadr(&mut self) -> SADR_W<0> {
        SADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA_CH9SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch9sadr](index.html) module"]
pub struct PDMA_CH9SADR_SPEC;
impl crate::RegisterSpec for PDMA_CH9SADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_ch9sadr::R](R) reader structure"]
impl crate::Readable for PDMA_CH9SADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_ch9sadr::W](W) writer structure"]
impl crate::Writable for PDMA_CH9SADR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDMA_CH9SADR to value 0"]
impl crate::Resettable for PDMA_CH9SADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

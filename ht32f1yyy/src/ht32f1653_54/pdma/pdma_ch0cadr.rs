#[doc = "Register `PDMA_CH0CADR` reader"]
pub struct R(crate::R<PDMA_CH0CADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_CH0CADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_CH0CADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_CH0CADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_CH0CADR` writer"]
pub struct W(crate::W<PDMA_CH0CADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_CH0CADR_SPEC>;
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
impl From<crate::W<PDMA_CH0CADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_CH0CADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDADR` reader - CDADR"]
pub type CDADR_R = crate::FieldReader<u16>;
#[doc = "Field `CDADR` writer - CDADR"]
pub type CDADR_W<'a, const O: u8> = crate::FieldWriter<'a, PDMA_CH0CADR_SPEC, 16, O, u16>;
#[doc = "Field `CSADR` reader - CSADR"]
pub type CSADR_R = crate::FieldReader<u16>;
#[doc = "Field `CSADR` writer - CSADR"]
pub type CSADR_W<'a, const O: u8> = crate::FieldWriter<'a, PDMA_CH0CADR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CDADR"]
    #[inline(always)]
    pub fn cdadr(&self) -> CDADR_R {
        CDADR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CSADR"]
    #[inline(always)]
    pub fn csadr(&self) -> CSADR_R {
        CSADR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CDADR"]
    #[inline(always)]
    #[must_use]
    pub fn cdadr(&mut self) -> CDADR_W<0> {
        CDADR_W::new(self)
    }
    #[doc = "Bits 16:31 - CSADR"]
    #[inline(always)]
    #[must_use]
    pub fn csadr(&mut self) -> CSADR_W<16> {
        CSADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA_CH0CADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch0cadr](index.html) module"]
pub struct PDMA_CH0CADR_SPEC;
impl crate::RegisterSpec for PDMA_CH0CADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_ch0cadr::R](R) reader structure"]
impl crate::Readable for PDMA_CH0CADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_ch0cadr::W](W) writer structure"]
impl crate::Writable for PDMA_CH0CADR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDMA_CH0CADR to value 0"]
impl crate::Resettable for PDMA_CH0CADR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

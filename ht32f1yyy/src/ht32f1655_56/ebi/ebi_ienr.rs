#[doc = "Register `EBI_IENR` reader"]
pub struct R(crate::R<EBI_IENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBI_IENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EBI_IENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EBI_IENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EBI_IENR` writer"]
pub struct W(crate::W<EBI_IENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBI_IENR_SPEC>;
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
impl From<crate::W<EBI_IENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EBI_IENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARDYTOIEN` reader - ARDYTOIEN"]
pub type ARDYTOIEN_R = crate::BitReader;
#[doc = "Field `ARDYTOIEN` writer - ARDYTOIEN"]
pub type ARDYTOIEN_W<'a, const O: u8> = crate::BitWriter<'a, EBI_IENR_SPEC, O>;
#[doc = "Field `ACCDISIEN` reader - ACCDISIEN"]
pub type ACCDISIEN_R = crate::BitReader;
#[doc = "Field `ACCDISIEN` writer - ACCDISIEN"]
pub type ACCDISIEN_W<'a, const O: u8> = crate::BitWriter<'a, EBI_IENR_SPEC, O>;
#[doc = "Field `ACCRSTIEN` reader - ACCRSTIEN"]
pub type ACCRSTIEN_R = crate::BitReader;
#[doc = "Field `ACCRSTIEN` writer - ACCRSTIEN"]
pub type ACCRSTIEN_W<'a, const O: u8> = crate::BitWriter<'a, EBI_IENR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ARDYTOIEN"]
    #[inline(always)]
    pub fn ardytoien(&self) -> ARDYTOIEN_R {
        ARDYTOIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ACCDISIEN"]
    #[inline(always)]
    pub fn accdisien(&self) -> ACCDISIEN_R {
        ACCDISIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACCRSTIEN"]
    #[inline(always)]
    pub fn accrstien(&self) -> ACCRSTIEN_R {
        ACCRSTIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARDYTOIEN"]
    #[inline(always)]
    #[must_use]
    pub fn ardytoien(&mut self) -> ARDYTOIEN_W<0> {
        ARDYTOIEN_W::new(self)
    }
    #[doc = "Bit 1 - ACCDISIEN"]
    #[inline(always)]
    #[must_use]
    pub fn accdisien(&mut self) -> ACCDISIEN_W<1> {
        ACCDISIEN_W::new(self)
    }
    #[doc = "Bit 2 - ACCRSTIEN"]
    #[inline(always)]
    #[must_use]
    pub fn accrstien(&mut self) -> ACCRSTIEN_W<2> {
        ACCRSTIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBI_IENR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_ienr](index.html) module"]
pub struct EBI_IENR_SPEC;
impl crate::RegisterSpec for EBI_IENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ebi_ienr::R](R) reader structure"]
impl crate::Readable for EBI_IENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ebi_ienr::W](W) writer structure"]
impl crate::Writable for EBI_IENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EBI_IENR to value 0"]
impl crate::Resettable for EBI_IENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

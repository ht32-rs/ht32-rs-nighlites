#[doc = "Register `SCTM_CHICFR` reader"]
pub struct R(crate::R<SCTM_CHICFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTM_CHICFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTM_CHICFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTM_CHICFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTM_CHICFR` writer"]
pub struct W(crate::W<SCTM_CHICFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTM_CHICFR_SPEC>;
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
impl From<crate::W<SCTM_CHICFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTM_CHICFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIF` reader - TIF"]
pub type TIF_R = crate::FieldReader;
#[doc = "Field `TIF` writer - TIF"]
pub type TIF_W<'a, const O: u8> = crate::FieldWriter<'a, SCTM_CHICFR_SPEC, 4, O>;
#[doc = "Field `CHCCS` reader - CHCCS"]
pub type CHCCS_R = crate::FieldReader;
#[doc = "Field `CHCCS` writer - CHCCS"]
pub type CHCCS_W<'a, const O: u8> = crate::FieldWriter<'a, SCTM_CHICFR_SPEC, 2, O>;
#[doc = "Field `CHPSC` reader - CHPSC"]
pub type CHPSC_R = crate::FieldReader;
#[doc = "Field `CHPSC` writer - CHPSC"]
pub type CHPSC_W<'a, const O: u8> = crate::FieldWriter<'a, SCTM_CHICFR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:3 - TIF"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - CHCCS"]
    #[inline(always)]
    pub fn chccs(&self) -> CHCCS_R {
        CHCCS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - CHPSC"]
    #[inline(always)]
    pub fn chpsc(&self) -> CHPSC_R {
        CHPSC_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TIF"]
    #[inline(always)]
    #[must_use]
    pub fn tif(&mut self) -> TIF_W<0> {
        TIF_W::new(self)
    }
    #[doc = "Bits 16:17 - CHCCS"]
    #[inline(always)]
    #[must_use]
    pub fn chccs(&mut self) -> CHCCS_W<16> {
        CHCCS_W::new(self)
    }
    #[doc = "Bits 18:19 - CHPSC"]
    #[inline(always)]
    #[must_use]
    pub fn chpsc(&mut self) -> CHPSC_W<18> {
        CHPSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCTM_CHICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctm_chicfr](index.html) module"]
pub struct SCTM_CHICFR_SPEC;
impl crate::RegisterSpec for SCTM_CHICFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sctm_chicfr::R](R) reader structure"]
impl crate::Readable for SCTM_CHICFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctm_chicfr::W](W) writer structure"]
impl crate::Writable for SCTM_CHICFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTM_CHICFR to value 0"]
impl crate::Resettable for SCTM_CHICFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

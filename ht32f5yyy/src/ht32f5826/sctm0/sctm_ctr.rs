#[doc = "Register `SCTM_CTR` reader"]
pub struct R(crate::R<SCTM_CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTM_CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTM_CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTM_CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTM_CTR` writer"]
pub struct W(crate::W<SCTM_CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTM_CTR_SPEC>;
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
impl From<crate::W<SCTM_CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTM_CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TME` reader - TME"]
pub type TME_R = crate::BitReader;
#[doc = "Field `TME` writer - TME"]
pub type TME_W<'a, const O: u8> = crate::BitWriter<'a, SCTM_CTR_SPEC, O>;
#[doc = "Field `CRBE` reader - CRBE"]
pub type CRBE_R = crate::BitReader;
#[doc = "Field `CRBE` writer - CRBE"]
pub type CRBE_W<'a, const O: u8> = crate::BitWriter<'a, SCTM_CTR_SPEC, O>;
#[doc = "Field `CHCCDS` reader - CHCCDS"]
pub type CHCCDS_R = crate::BitReader;
#[doc = "Field `CHCCDS` writer - CHCCDS"]
pub type CHCCDS_W<'a, const O: u8> = crate::BitWriter<'a, SCTM_CTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - TME"]
    #[inline(always)]
    pub fn tme(&self) -> TME_R {
        TME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRBE"]
    #[inline(always)]
    pub fn crbe(&self) -> CRBE_R {
        CRBE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - CHCCDS"]
    #[inline(always)]
    pub fn chccds(&self) -> CHCCDS_R {
        CHCCDS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TME"]
    #[inline(always)]
    #[must_use]
    pub fn tme(&mut self) -> TME_W<0> {
        TME_W::new(self)
    }
    #[doc = "Bit 1 - CRBE"]
    #[inline(always)]
    #[must_use]
    pub fn crbe(&mut self) -> CRBE_W<1> {
        CRBE_W::new(self)
    }
    #[doc = "Bit 16 - CHCCDS"]
    #[inline(always)]
    #[must_use]
    pub fn chccds(&mut self) -> CHCCDS_W<16> {
        CHCCDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCTM_CTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctm_ctr](index.html) module"]
pub struct SCTM_CTR_SPEC;
impl crate::RegisterSpec for SCTM_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sctm_ctr::R](R) reader structure"]
impl crate::Readable for SCTM_CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctm_ctr::W](W) writer structure"]
impl crate::Writable for SCTM_CTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTM_CTR to value 0"]
impl crate::Resettable for SCTM_CTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

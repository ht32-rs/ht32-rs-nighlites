#[doc = "Register `SCTM_MDCFR` reader"]
pub struct R(crate::R<SCTM_MDCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTM_MDCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTM_MDCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTM_MDCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTM_MDCFR` writer"]
pub struct W(crate::W<SCTM_MDCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTM_MDCFR_SPEC>;
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
impl From<crate::W<SCTM_MDCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTM_MDCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSE` reader - TSE"]
pub type TSE_R = crate::BitReader;
#[doc = "Field `TSE` writer - TSE"]
pub type TSE_W<'a, const O: u8> = crate::BitWriter<'a, SCTM_MDCFR_SPEC, O>;
#[doc = "Field `SMSEL` reader - SMSEL"]
pub type SMSEL_R = crate::FieldReader;
#[doc = "Field `SMSEL` writer - SMSEL"]
pub type SMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, SCTM_MDCFR_SPEC, 3, O>;
#[doc = "Field `MMSEL` reader - MMSEL"]
pub type MMSEL_R = crate::FieldReader;
#[doc = "Field `MMSEL` writer - MMSEL"]
pub type MMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, SCTM_MDCFR_SPEC, 3, O>;
#[doc = "Field `SPMSET` reader - SPMSET"]
pub type SPMSET_R = crate::BitReader;
#[doc = "Field `SPMSET` writer - SPMSET"]
pub type SPMSET_W<'a, const O: u8> = crate::BitWriter<'a, SCTM_MDCFR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - TSE"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - SMSEL"]
    #[inline(always)]
    pub fn smsel(&self) -> SMSEL_R {
        SMSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - MMSEL"]
    #[inline(always)]
    pub fn mmsel(&self) -> MMSEL_R {
        MMSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - SPMSET"]
    #[inline(always)]
    pub fn spmset(&self) -> SPMSET_R {
        SPMSET_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TSE"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<0> {
        TSE_W::new(self)
    }
    #[doc = "Bits 8:10 - SMSEL"]
    #[inline(always)]
    #[must_use]
    pub fn smsel(&mut self) -> SMSEL_W<8> {
        SMSEL_W::new(self)
    }
    #[doc = "Bits 16:18 - MMSEL"]
    #[inline(always)]
    #[must_use]
    pub fn mmsel(&mut self) -> MMSEL_W<16> {
        MMSEL_W::new(self)
    }
    #[doc = "Bit 24 - SPMSET"]
    #[inline(always)]
    #[must_use]
    pub fn spmset(&mut self) -> SPMSET_W<24> {
        SPMSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCTM_MDCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctm_mdcfr](index.html) module"]
pub struct SCTM_MDCFR_SPEC;
impl crate::RegisterSpec for SCTM_MDCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sctm_mdcfr::R](R) reader structure"]
impl crate::Readable for SCTM_MDCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctm_mdcfr::W](W) writer structure"]
impl crate::Writable for SCTM_MDCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTM_MDCFR to value 0"]
impl crate::Resettable for SCTM_MDCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

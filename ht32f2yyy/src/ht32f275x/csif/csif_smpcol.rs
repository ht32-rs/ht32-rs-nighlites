#[doc = "Register `CSIF_SMPCOL` reader"]
pub struct R(crate::R<CSIF_SMPCOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIF_SMPCOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIF_SMPCOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIF_SMPCOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSIF_SMPCOL` writer"]
pub struct W(crate::W<CSIF_SMPCOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIF_SMPCOL_SPEC>;
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
impl From<crate::W<CSIF_SMPCOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIF_SMPCOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSM` reader - CSM"]
pub type CSM_R = crate::FieldReader<u32>;
#[doc = "Field `CSM` writer - CSM"]
pub type CSM_W<'a, const O: u8> = crate::FieldWriter<'a, CSIF_SMPCOL_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CSM"]
    #[inline(always)]
    pub fn csm(&self) -> CSM_R {
        CSM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSM"]
    #[inline(always)]
    #[must_use]
    pub fn csm(&mut self) -> CSM_W<0> {
        CSM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIF_SMPCOL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_smpcol](index.html) module"]
pub struct CSIF_SMPCOL_SPEC;
impl crate::RegisterSpec for CSIF_SMPCOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csif_smpcol::R](R) reader structure"]
impl crate::Readable for CSIF_SMPCOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csif_smpcol::W](W) writer structure"]
impl crate::Writable for CSIF_SMPCOL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSIF_SMPCOL to value 0"]
impl crate::Resettable for CSIF_SMPCOL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

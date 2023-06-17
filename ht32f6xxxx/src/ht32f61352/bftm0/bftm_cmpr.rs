#[doc = "Register `BFTM_CMPR` reader"]
pub struct R(crate::R<BFTM_CMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFTM_CMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFTM_CMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFTM_CMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFTM_CMPR` writer"]
pub struct W(crate::W<BFTM_CMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFTM_CMPR_SPEC>;
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
impl From<crate::W<BFTM_CMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFTM_CMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP` reader - CMP"]
pub type CMP_R = crate::FieldReader<u32>;
#[doc = "Field `CMP` writer - CMP"]
pub type CMP_W<'a, const O: u8> = crate::FieldWriter<'a, BFTM_CMPR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CMP"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CMP"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<0> {
        CMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BFTM_CMPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bftm_cmpr](index.html) module"]
pub struct BFTM_CMPR_SPEC;
impl crate::RegisterSpec for BFTM_CMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bftm_cmpr::R](R) reader structure"]
impl crate::Readable for BFTM_CMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bftm_cmpr::W](W) writer structure"]
impl crate::Writable for BFTM_CMPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BFTM_CMPR to value 0"]
impl crate::Resettable for BFTM_CMPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

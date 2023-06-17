#[doc = "Register `BFTM_CNTR` reader"]
pub struct R(crate::R<BFTM_CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFTM_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFTM_CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFTM_CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFTM_CNTR` writer"]
pub struct W(crate::W<BFTM_CNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFTM_CNTR_SPEC>;
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
impl From<crate::W<BFTM_CNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFTM_CNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - CNT"]
pub type CNT_R = crate::FieldReader<u32>;
#[doc = "Field `CNT` writer - CNT"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, BFTM_CNTR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CNT"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BFTM_CNTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bftm_cntr](index.html) module"]
pub struct BFTM_CNTR_SPEC;
impl crate::RegisterSpec for BFTM_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bftm_cntr::R](R) reader structure"]
impl crate::Readable for BFTM_CNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bftm_cntr::W](W) writer structure"]
impl crate::Writable for BFTM_CNTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BFTM_CNTR to value 0"]
impl crate::Resettable for BFTM_CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

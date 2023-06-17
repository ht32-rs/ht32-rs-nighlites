#[doc = "Register `BFTMCMPR` reader"]
pub struct R(crate::R<BFTMCMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFTMCMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFTMCMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFTMCMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFTMCMPR` writer"]
pub struct W(crate::W<BFTMCMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFTMCMPR_SPEC>;
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
impl From<crate::W<BFTMCMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFTMCMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP` reader - CMP"]
pub type CMP_R = crate::FieldReader<u32>;
#[doc = "Field `CMP` writer - CMP"]
pub type CMP_W<'a, const O: u8> = crate::FieldWriter<'a, BFTMCMPR_SPEC, 32, O, u32>;
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
#[doc = "BFTMCMPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bftmcmpr](index.html) module"]
pub struct BFTMCMPR_SPEC;
impl crate::RegisterSpec for BFTMCMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bftmcmpr::R](R) reader structure"]
impl crate::Readable for BFTMCMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bftmcmpr::W](W) writer structure"]
impl crate::Writable for BFTMCMPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BFTMCMPR to value 0"]
impl crate::Resettable for BFTMCMPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

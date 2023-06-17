#[doc = "Register `USRTSTR` reader"]
pub struct R(crate::R<USRTSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USRTSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USRTSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USRTSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USRTSTR` writer"]
pub struct W(crate::W<USRTSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USRTSTR_SPEC>;
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
impl From<crate::W<USRTSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USRTSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBM` reader - LBM"]
pub type LBM_R = crate::FieldReader;
#[doc = "Field `LBM` writer - LBM"]
pub type LBM_W<'a, const O: u8> = crate::FieldWriter<'a, USRTSTR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - LBM"]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LBM"]
    #[inline(always)]
    #[must_use]
    pub fn lbm(&mut self) -> LBM_W<0> {
        LBM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USRTSTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usrtstr](index.html) module"]
pub struct USRTSTR_SPEC;
impl crate::RegisterSpec for USRTSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usrtstr::R](R) reader structure"]
impl crate::Readable for USRTSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usrtstr::W](W) writer structure"]
impl crate::Writable for USRTSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USRTSTR to value 0"]
impl crate::Resettable for USRTSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `CKCU_GCIR` reader"]
pub struct R(crate::R<CKCU_GCIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_GCIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_GCIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_GCIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_GCIR` writer"]
pub struct W(crate::W<CKCU_GCIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_GCIR_SPEC>;
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
impl From<crate::W<CKCU_GCIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_GCIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKSF` reader - CKSF"]
pub type CKSF_R = crate::BitReader;
#[doc = "Field `CKSF` writer - CKSF"]
pub type CKSF_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_GCIR_SPEC, O>;
#[doc = "Field `CKSIE` reader - CKSIE"]
pub type CKSIE_R = crate::BitReader;
#[doc = "Field `CKSIE` writer - CKSIE"]
pub type CKSIE_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_GCIR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CKSF"]
    #[inline(always)]
    pub fn cksf(&self) -> CKSF_R {
        CKSF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - CKSIE"]
    #[inline(always)]
    pub fn cksie(&self) -> CKSIE_R {
        CKSIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CKSF"]
    #[inline(always)]
    #[must_use]
    pub fn cksf(&mut self) -> CKSF_W<0> {
        CKSF_W::new(self)
    }
    #[doc = "Bit 16 - CKSIE"]
    #[inline(always)]
    #[must_use]
    pub fn cksie(&mut self) -> CKSIE_W<16> {
        CKSIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_GCIR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_gcir](index.html) module"]
pub struct CKCU_GCIR_SPEC;
impl crate::RegisterSpec for CKCU_GCIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_gcir::R](R) reader structure"]
impl crate::Readable for CKCU_GCIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_gcir::W](W) writer structure"]
impl crate::Writable for CKCU_GCIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_GCIR to value 0"]
impl crate::Resettable for CKCU_GCIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

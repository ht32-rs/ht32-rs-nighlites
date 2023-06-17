#[doc = "Register `GCIR` reader"]
pub struct R(crate::R<GCIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCIR` writer"]
pub struct W(crate::W<GCIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCIR_SPEC>;
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
impl From<crate::W<GCIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKSF` reader - CKSF"]
pub type CKSF_R = crate::BitReader;
#[doc = "Field `CKSF` writer - CKSF"]
pub type CKSF_W<'a, const O: u8> = crate::BitWriter<'a, GCIR_SPEC, O>;
#[doc = "Field `CKSIE` reader - CKSIE"]
pub type CKSIE_R = crate::BitReader;
#[doc = "Field `CKSIE` writer - CKSIE"]
pub type CKSIE_W<'a, const O: u8> = crate::BitWriter<'a, GCIR_SPEC, O>;
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
#[doc = "GCIR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcir](index.html) module"]
pub struct GCIR_SPEC;
impl crate::RegisterSpec for GCIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcir::R](R) reader structure"]
impl crate::Readable for GCIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcir::W](W) writer structure"]
impl crate::Writable for GCIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCIR to value 0"]
impl crate::Resettable for GCIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `CSIF_ENR` reader"]
pub struct R(crate::R<CSIF_ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIF_ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIF_ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIF_ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSIF_ENR` writer"]
pub struct W(crate::W<CSIF_ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIF_ENR_SPEC>;
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
impl From<crate::W<CSIF_ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIF_ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSIF_EN` reader - CSIF_EN"]
pub type CSIF_EN_R = crate::BitReader;
#[doc = "Field `CSIF_EN` writer - CSIF_EN"]
pub type CSIF_EN_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_ENR_SPEC, O>;
impl R {
    #[doc = "Bit 31 - CSIF_EN"]
    #[inline(always)]
    pub fn csif_en(&self) -> CSIF_EN_R {
        CSIF_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - CSIF_EN"]
    #[inline(always)]
    #[must_use]
    pub fn csif_en(&mut self) -> CSIF_EN_W<31> {
        CSIF_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIF_ENR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_enr](index.html) module"]
pub struct CSIF_ENR_SPEC;
impl crate::RegisterSpec for CSIF_ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csif_enr::R](R) reader structure"]
impl crate::Readable for CSIF_ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csif_enr::W](W) writer structure"]
impl crate::Writable for CSIF_ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSIF_ENR to value 0"]
impl crate::Resettable for CSIF_ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

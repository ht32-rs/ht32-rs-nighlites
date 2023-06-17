#[doc = "Register `EBI_IFR` reader"]
pub struct R(crate::R<EBI_IFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBI_IFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EBI_IFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EBI_IFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EBI_IFR` writer"]
pub struct W(crate::W<EBI_IFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBI_IFR_SPEC>;
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
impl From<crate::W<EBI_IFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EBI_IFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARDYTOIF` reader - ARDYTOIF"]
pub type ARDYTOIF_R = crate::BitReader;
#[doc = "Field `ARDYTOIF` writer - ARDYTOIF"]
pub type ARDYTOIF_W<'a, const O: u8> = crate::BitWriter<'a, EBI_IFR_SPEC, O>;
#[doc = "Field `ACCERRIF` reader - ACCERRIF"]
pub type ACCERRIF_R = crate::BitReader;
#[doc = "Field `ACCERRIF` writer - ACCERRIF"]
pub type ACCERRIF_W<'a, const O: u8> = crate::BitWriter<'a, EBI_IFR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ARDYTOIF"]
    #[inline(always)]
    pub fn ardytoif(&self) -> ARDYTOIF_R {
        ARDYTOIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ACCERRIF"]
    #[inline(always)]
    pub fn accerrif(&self) -> ACCERRIF_R {
        ACCERRIF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARDYTOIF"]
    #[inline(always)]
    #[must_use]
    pub fn ardytoif(&mut self) -> ARDYTOIF_W<0> {
        ARDYTOIF_W::new(self)
    }
    #[doc = "Bit 1 - ACCERRIF"]
    #[inline(always)]
    #[must_use]
    pub fn accerrif(&mut self) -> ACCERRIF_W<1> {
        ACCERRIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBI_IFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_ifr](index.html) module"]
pub struct EBI_IFR_SPEC;
impl crate::RegisterSpec for EBI_IFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ebi_ifr::R](R) reader structure"]
impl crate::Readable for EBI_IFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ebi_ifr::W](W) writer structure"]
impl crate::Writable for EBI_IFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EBI_IFR to value 0"]
impl crate::Resettable for EBI_IFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

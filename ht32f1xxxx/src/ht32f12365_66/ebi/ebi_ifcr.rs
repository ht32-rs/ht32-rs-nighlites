#[doc = "Register `EBI_IFCR` reader"]
pub struct R(crate::R<EBI_IFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBI_IFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EBI_IFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EBI_IFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EBI_IFCR` writer"]
pub struct W(crate::W<EBI_IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBI_IFCR_SPEC>;
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
impl From<crate::W<EBI_IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EBI_IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARDYTOIC` reader - ARDYTOIC"]
pub type ARDYTOIC_R = crate::BitReader;
#[doc = "Field `ARDYTOIC` writer - ARDYTOIC"]
pub type ARDYTOIC_W<'a, const O: u8> = crate::BitWriter<'a, EBI_IFCR_SPEC, O>;
#[doc = "Field `ACCERRIC` reader - ACCERRIC"]
pub type ACCERRIC_R = crate::BitReader;
#[doc = "Field `ACCERRIC` writer - ACCERRIC"]
pub type ACCERRIC_W<'a, const O: u8> = crate::BitWriter<'a, EBI_IFCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ARDYTOIC"]
    #[inline(always)]
    pub fn ardytoic(&self) -> ARDYTOIC_R {
        ARDYTOIC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ACCERRIC"]
    #[inline(always)]
    pub fn accerric(&self) -> ACCERRIC_R {
        ACCERRIC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ARDYTOIC"]
    #[inline(always)]
    #[must_use]
    pub fn ardytoic(&mut self) -> ARDYTOIC_W<0> {
        ARDYTOIC_W::new(self)
    }
    #[doc = "Bit 1 - ACCERRIC"]
    #[inline(always)]
    #[must_use]
    pub fn accerric(&mut self) -> ACCERRIC_W<1> {
        ACCERRIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBI_IFCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_ifcr](index.html) module"]
pub struct EBI_IFCR_SPEC;
impl crate::RegisterSpec for EBI_IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ebi_ifcr::R](R) reader structure"]
impl crate::Readable for EBI_IFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ebi_ifcr::W](W) writer structure"]
impl crate::Writable for EBI_IFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EBI_IFCR to value 0"]
impl crate::Resettable for EBI_IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

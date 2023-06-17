#[doc = "Register `SCI_PSC` reader"]
pub struct R(crate::R<SCI_PSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCI_PSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCI_PSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCI_PSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCI_PSC` writer"]
pub struct W(crate::W<SCI_PSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCI_PSC_SPEC>;
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
impl From<crate::W<SCI_PSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCI_PSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSC` reader - PSC"]
pub type PSC_R = crate::FieldReader;
#[doc = "Field `PSC` writer - PSC"]
pub type PSC_W<'a, const O: u8> = crate::FieldWriter<'a, SCI_PSC_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:5 - PSC"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PSC"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<0> {
        PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCI_PSC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_psc](index.html) module"]
pub struct SCI_PSC_SPEC;
impl crate::RegisterSpec for SCI_PSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sci_psc::R](R) reader structure"]
impl crate::Readable for SCI_PSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sci_psc::W](W) writer structure"]
impl crate::Writable for SCI_PSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCI_PSC to value 0"]
impl crate::Resettable for SCI_PSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `SCI_WTR` reader"]
pub struct R(crate::R<SCI_WTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCI_WTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCI_WTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCI_WTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCI_WTR` writer"]
pub struct W(crate::W<SCI_WTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCI_WTR_SPEC>;
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
impl From<crate::W<SCI_WTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCI_WTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WT` reader - WT"]
pub type WT_R = crate::FieldReader<u32>;
#[doc = "Field `WT` writer - WT"]
pub type WT_W<'a, const O: u8> = crate::FieldWriter<'a, SCI_WTR_SPEC, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - WT"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - WT"]
    #[inline(always)]
    #[must_use]
    pub fn wt(&mut self) -> WT_W<0> {
        WT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCI_WTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_wtr](index.html) module"]
pub struct SCI_WTR_SPEC;
impl crate::RegisterSpec for SCI_WTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sci_wtr::R](R) reader structure"]
impl crate::Readable for SCI_WTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sci_wtr::W](W) writer structure"]
impl crate::Writable for SCI_WTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCI_WTR to value 0"]
impl crate::Resettable for SCI_WTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

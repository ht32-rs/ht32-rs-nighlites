#[doc = "Register `PDSRR` reader"]
pub struct R(crate::R<PDSRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDSRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDSRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDSRR` writer"]
pub struct W(crate::W<PDSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDSRR_SPEC>;
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
impl From<crate::W<PDSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDSRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDSET` reader - PDSET"]
pub type PDSET_R = crate::FieldReader;
#[doc = "Field `PDSET` writer - PDSET"]
pub type PDSET_W<'a, const O: u8> = crate::FieldWriter<'a, PDSRR_SPEC, 4, O>;
#[doc = "Field `PDRST` reader - PDRST"]
pub type PDRST_R = crate::FieldReader;
#[doc = "Field `PDRST` writer - PDRST"]
pub type PDRST_W<'a, const O: u8> = crate::FieldWriter<'a, PDSRR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PDSET"]
    #[inline(always)]
    pub fn pdset(&self) -> PDSET_R {
        PDSET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PDRST"]
    #[inline(always)]
    pub fn pdrst(&self) -> PDRST_R {
        PDRST_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDSET"]
    #[inline(always)]
    #[must_use]
    pub fn pdset(&mut self) -> PDSET_W<0> {
        PDSET_W::new(self)
    }
    #[doc = "Bits 16:19 - PDRST"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst(&mut self) -> PDRST_W<16> {
        PDRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDSRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsrr](index.html) module"]
pub struct PDSRR_SPEC;
impl crate::RegisterSpec for PDSRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdsrr::R](R) reader structure"]
impl crate::Readable for PDSRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdsrr::W](W) writer structure"]
impl crate::Writable for PDSRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDSRR to value 0"]
impl crate::Resettable for PDSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

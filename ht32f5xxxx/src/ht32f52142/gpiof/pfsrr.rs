#[doc = "Register `PFSRR` reader"]
pub struct R(crate::R<PFSRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFSRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFSRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFSRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFSRR` writer"]
pub struct W(crate::W<PFSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFSRR_SPEC>;
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
impl From<crate::W<PFSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFSRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFSET0` reader - PFSET0"]
pub type PFSET0_R = crate::BitReader;
#[doc = "Field `PFSET0` writer - PFSET0"]
pub type PFSET0_W<'a, const O: u8> = crate::BitWriter<'a, PFSRR_SPEC, O>;
#[doc = "Field `PFSET1` reader - PFSET1"]
pub type PFSET1_R = crate::BitReader;
#[doc = "Field `PFSET1` writer - PFSET1"]
pub type PFSET1_W<'a, const O: u8> = crate::BitWriter<'a, PFSRR_SPEC, O>;
#[doc = "Field `PFRST0` reader - PFRST0"]
pub type PFRST0_R = crate::BitReader;
#[doc = "Field `PFRST0` writer - PFRST0"]
pub type PFRST0_W<'a, const O: u8> = crate::BitWriter<'a, PFSRR_SPEC, O>;
#[doc = "Field `PFRST1` reader - PFRST1"]
pub type PFRST1_R = crate::BitReader;
#[doc = "Field `PFRST1` writer - PFRST1"]
pub type PFRST1_W<'a, const O: u8> = crate::BitWriter<'a, PFSRR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PFSET0"]
    #[inline(always)]
    pub fn pfset0(&self) -> PFSET0_R {
        PFSET0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PFSET1"]
    #[inline(always)]
    pub fn pfset1(&self) -> PFSET1_R {
        PFSET1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - PFRST0"]
    #[inline(always)]
    pub fn pfrst0(&self) -> PFRST0_R {
        PFRST0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PFRST1"]
    #[inline(always)]
    pub fn pfrst1(&self) -> PFRST1_R {
        PFRST1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PFSET0"]
    #[inline(always)]
    #[must_use]
    pub fn pfset0(&mut self) -> PFSET0_W<0> {
        PFSET0_W::new(self)
    }
    #[doc = "Bit 1 - PFSET1"]
    #[inline(always)]
    #[must_use]
    pub fn pfset1(&mut self) -> PFSET1_W<1> {
        PFSET1_W::new(self)
    }
    #[doc = "Bit 16 - PFRST0"]
    #[inline(always)]
    #[must_use]
    pub fn pfrst0(&mut self) -> PFRST0_W<16> {
        PFRST0_W::new(self)
    }
    #[doc = "Bit 17 - PFRST1"]
    #[inline(always)]
    #[must_use]
    pub fn pfrst1(&mut self) -> PFRST1_W<17> {
        PFRST1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PFSRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfsrr](index.html) module"]
pub struct PFSRR_SPEC;
impl crate::RegisterSpec for PFSRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfsrr::R](R) reader structure"]
impl crate::Readable for PFSRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfsrr::W](W) writer structure"]
impl crate::Writable for PFSRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFSRR to value 0"]
impl crate::Resettable for PFSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

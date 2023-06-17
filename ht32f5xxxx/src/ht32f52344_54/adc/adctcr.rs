#[doc = "Register `ADCTCR` reader"]
pub struct R(crate::R<ADCTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCTCR` writer"]
pub struct W(crate::W<ADCTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCTCR_SPEC>;
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
impl From<crate::W<ADCTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADSW` reader - ADSW"]
pub type ADSW_R = crate::BitReader;
#[doc = "Field `ADSW` writer - ADSW"]
pub type ADSW_W<'a, const O: u8> = crate::BitWriter<'a, ADCTCR_SPEC, O>;
#[doc = "Field `ADEXTI` reader - ADEXTI"]
pub type ADEXTI_R = crate::BitReader;
#[doc = "Field `ADEXTI` writer - ADEXTI"]
pub type ADEXTI_W<'a, const O: u8> = crate::BitWriter<'a, ADCTCR_SPEC, O>;
#[doc = "Field `TM` reader - TM"]
pub type TM_R = crate::BitReader;
#[doc = "Field `TM` writer - TM"]
pub type TM_W<'a, const O: u8> = crate::BitWriter<'a, ADCTCR_SPEC, O>;
#[doc = "Field `BFTM` reader - BFTM"]
pub type BFTM_R = crate::BitReader;
#[doc = "Field `BFTM` writer - BFTM"]
pub type BFTM_W<'a, const O: u8> = crate::BitWriter<'a, ADCTCR_SPEC, O>;
#[doc = "Field `CMP` reader - CMP"]
pub type CMP_R = crate::BitReader;
#[doc = "Field `CMP` writer - CMP"]
pub type CMP_W<'a, const O: u8> = crate::BitWriter<'a, ADCTCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ADSW"]
    #[inline(always)]
    pub fn adsw(&self) -> ADSW_R {
        ADSW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADEXTI"]
    #[inline(always)]
    pub fn adexti(&self) -> ADEXTI_R {
        ADEXTI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TM"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BFTM"]
    #[inline(always)]
    pub fn bftm(&self) -> BFTM_R {
        BFTM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMP"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADSW"]
    #[inline(always)]
    #[must_use]
    pub fn adsw(&mut self) -> ADSW_W<0> {
        ADSW_W::new(self)
    }
    #[doc = "Bit 1 - ADEXTI"]
    #[inline(always)]
    #[must_use]
    pub fn adexti(&mut self) -> ADEXTI_W<1> {
        ADEXTI_W::new(self)
    }
    #[doc = "Bit 2 - TM"]
    #[inline(always)]
    #[must_use]
    pub fn tm(&mut self) -> TM_W<2> {
        TM_W::new(self)
    }
    #[doc = "Bit 3 - BFTM"]
    #[inline(always)]
    #[must_use]
    pub fn bftm(&mut self) -> BFTM_W<3> {
        BFTM_W::new(self)
    }
    #[doc = "Bit 4 - CMP"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<4> {
        CMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADCTCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adctcr](index.html) module"]
pub struct ADCTCR_SPEC;
impl crate::RegisterSpec for ADCTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adctcr::R](R) reader structure"]
impl crate::Readable for ADCTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adctcr::W](W) writer structure"]
impl crate::Writable for ADCTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCTCR to value 0"]
impl crate::Resettable for ADCTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

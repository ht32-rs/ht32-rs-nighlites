#[doc = "Register `ADCISR` reader"]
pub struct R(crate::R<ADCISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCISR` writer"]
pub struct W(crate::W<ADCISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCISR_SPEC>;
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
impl From<crate::W<ADCISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADISRS` reader - ADISRS"]
pub type ADISRS_R = crate::BitReader;
#[doc = "Field `ADISRS` writer - ADISRS"]
pub type ADISRS_W<'a, const O: u8> = crate::BitWriter<'a, ADCISR_SPEC, O>;
#[doc = "Field `ADISRG` reader - ADISRG"]
pub type ADISRG_R = crate::BitReader;
#[doc = "Field `ADISRG` writer - ADISRG"]
pub type ADISRG_W<'a, const O: u8> = crate::BitWriter<'a, ADCISR_SPEC, O>;
#[doc = "Field `ADISRC` reader - ADISRC"]
pub type ADISRC_R = crate::BitReader;
#[doc = "Field `ADISRC` writer - ADISRC"]
pub type ADISRC_W<'a, const O: u8> = crate::BitWriter<'a, ADCISR_SPEC, O>;
#[doc = "Field `ADISRL` reader - ADISRL"]
pub type ADISRL_R = crate::BitReader;
#[doc = "Field `ADISRL` writer - ADISRL"]
pub type ADISRL_W<'a, const O: u8> = crate::BitWriter<'a, ADCISR_SPEC, O>;
#[doc = "Field `ADISRU` reader - ADISRU"]
pub type ADISRU_R = crate::BitReader;
#[doc = "Field `ADISRU` writer - ADISRU"]
pub type ADISRU_W<'a, const O: u8> = crate::BitWriter<'a, ADCISR_SPEC, O>;
#[doc = "Field `ADISRO` reader - ADISRO"]
pub type ADISRO_R = crate::BitReader;
#[doc = "Field `ADISRO` writer - ADISRO"]
pub type ADISRO_W<'a, const O: u8> = crate::BitWriter<'a, ADCISR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ADISRS"]
    #[inline(always)]
    pub fn adisrs(&self) -> ADISRS_R {
        ADISRS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADISRG"]
    #[inline(always)]
    pub fn adisrg(&self) -> ADISRG_R {
        ADISRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADISRC"]
    #[inline(always)]
    pub fn adisrc(&self) -> ADISRC_R {
        ADISRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - ADISRL"]
    #[inline(always)]
    pub fn adisrl(&self) -> ADISRL_R {
        ADISRL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADISRU"]
    #[inline(always)]
    pub fn adisru(&self) -> ADISRU_R {
        ADISRU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - ADISRO"]
    #[inline(always)]
    pub fn adisro(&self) -> ADISRO_R {
        ADISRO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADISRS"]
    #[inline(always)]
    #[must_use]
    pub fn adisrs(&mut self) -> ADISRS_W<0> {
        ADISRS_W::new(self)
    }
    #[doc = "Bit 1 - ADISRG"]
    #[inline(always)]
    #[must_use]
    pub fn adisrg(&mut self) -> ADISRG_W<1> {
        ADISRG_W::new(self)
    }
    #[doc = "Bit 2 - ADISRC"]
    #[inline(always)]
    #[must_use]
    pub fn adisrc(&mut self) -> ADISRC_W<2> {
        ADISRC_W::new(self)
    }
    #[doc = "Bit 16 - ADISRL"]
    #[inline(always)]
    #[must_use]
    pub fn adisrl(&mut self) -> ADISRL_W<16> {
        ADISRL_W::new(self)
    }
    #[doc = "Bit 17 - ADISRU"]
    #[inline(always)]
    #[must_use]
    pub fn adisru(&mut self) -> ADISRU_W<17> {
        ADISRU_W::new(self)
    }
    #[doc = "Bit 24 - ADISRO"]
    #[inline(always)]
    #[must_use]
    pub fn adisro(&mut self) -> ADISRO_W<24> {
        ADISRO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADCISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcisr](index.html) module"]
pub struct ADCISR_SPEC;
impl crate::RegisterSpec for ADCISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcisr::R](R) reader structure"]
impl crate::Readable for ADCISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcisr::W](W) writer structure"]
impl crate::Writable for ADCISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCISR to value 0"]
impl crate::Resettable for ADCISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

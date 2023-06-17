#[doc = "Register `PWM_CNTCFR` reader"]
pub struct R(crate::R<PWM_CNTCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CNTCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CNTCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CNTCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CNTCFR` writer"]
pub struct W(crate::W<PWM_CNTCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CNTCFR_SPEC>;
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
impl From<crate::W<PWM_CNTCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CNTCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UEVDIS` reader - UEVDIS"]
pub type UEVDIS_R = crate::BitReader;
#[doc = "Field `UEVDIS` writer - UEVDIS"]
pub type UEVDIS_W<'a, const O: u8> = crate::BitWriter<'a, PWM_CNTCFR_SPEC, O>;
#[doc = "Field `UGDIS` reader - UGDIS"]
pub type UGDIS_R = crate::BitReader;
#[doc = "Field `UGDIS` writer - UGDIS"]
pub type UGDIS_W<'a, const O: u8> = crate::BitWriter<'a, PWM_CNTCFR_SPEC, O>;
#[doc = "Field `CMSEL` reader - CMSEL"]
pub type CMSEL_R = crate::FieldReader;
#[doc = "Field `CMSEL` writer - CMSEL"]
pub type CMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, PWM_CNTCFR_SPEC, 2, O>;
#[doc = "Field `DIR` reader - DIR"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - DIR"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, PWM_CNTCFR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - UEVDIS"]
    #[inline(always)]
    pub fn uevdis(&self) -> UEVDIS_R {
        UEVDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UGDIS"]
    #[inline(always)]
    pub fn ugdis(&self) -> UGDIS_R {
        UGDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:17 - CMSEL"]
    #[inline(always)]
    pub fn cmsel(&self) -> CMSEL_R {
        CMSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UEVDIS"]
    #[inline(always)]
    #[must_use]
    pub fn uevdis(&mut self) -> UEVDIS_W<0> {
        UEVDIS_W::new(self)
    }
    #[doc = "Bit 1 - UGDIS"]
    #[inline(always)]
    #[must_use]
    pub fn ugdis(&mut self) -> UGDIS_W<1> {
        UGDIS_W::new(self)
    }
    #[doc = "Bits 16:17 - CMSEL"]
    #[inline(always)]
    #[must_use]
    pub fn cmsel(&mut self) -> CMSEL_W<16> {
        CMSEL_W::new(self)
    }
    #[doc = "Bit 24 - DIR"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<24> {
        DIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM_CNTCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cntcfr](index.html) module"]
pub struct PWM_CNTCFR_SPEC;
impl crate::RegisterSpec for PWM_CNTCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_cntcfr::R](R) reader structure"]
impl crate::Readable for PWM_CNTCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_cntcfr::W](W) writer structure"]
impl crate::Writable for PWM_CNTCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWM_CNTCFR to value 0"]
impl crate::Resettable for PWM_CNTCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

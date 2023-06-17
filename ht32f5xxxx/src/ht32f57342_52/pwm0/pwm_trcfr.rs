#[doc = "Register `PWM_TRCFR` reader"]
pub struct R(crate::R<PWM_TRCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_TRCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_TRCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_TRCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_TRCFR` writer"]
pub struct W(crate::W<PWM_TRCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_TRCFR_SPEC>;
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
impl From<crate::W<PWM_TRCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_TRCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRSEL` reader - TRSEL"]
pub type TRSEL_R = crate::FieldReader;
#[doc = "Field `TRSEL` writer - TRSEL"]
pub type TRSEL_W<'a, const O: u8> = crate::FieldWriter<'a, PWM_TRCFR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - TRSEL"]
    #[inline(always)]
    pub fn trsel(&self) -> TRSEL_R {
        TRSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TRSEL"]
    #[inline(always)]
    #[must_use]
    pub fn trsel(&mut self) -> TRSEL_W<0> {
        TRSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM_TRCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_trcfr](index.html) module"]
pub struct PWM_TRCFR_SPEC;
impl crate::RegisterSpec for PWM_TRCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_trcfr::R](R) reader structure"]
impl crate::Readable for PWM_TRCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_trcfr::W](W) writer structure"]
impl crate::Writable for PWM_TRCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWM_TRCFR to value 0"]
impl crate::Resettable for PWM_TRCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

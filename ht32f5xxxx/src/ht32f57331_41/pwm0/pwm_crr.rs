#[doc = "Register `PWM_CRR` reader"]
pub struct R(crate::R<PWM_CRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CRR` writer"]
pub struct W(crate::W<PWM_CRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CRR_SPEC>;
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
impl From<crate::W<PWM_CRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRV` reader - CRV"]
pub type CRV_R = crate::FieldReader<u16>;
#[doc = "Field `CRV` writer - CRV"]
pub type CRV_W<'a, const O: u8> = crate::FieldWriter<'a, PWM_CRR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CRV"]
    #[inline(always)]
    pub fn crv(&self) -> CRV_R {
        CRV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRV"]
    #[inline(always)]
    #[must_use]
    pub fn crv(&mut self) -> CRV_W<0> {
        CRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM_CRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_crr](index.html) module"]
pub struct PWM_CRR_SPEC;
impl crate::RegisterSpec for PWM_CRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_crr::R](R) reader structure"]
impl crate::Readable for PWM_CRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_crr::W](W) writer structure"]
impl crate::Writable for PWM_CRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWM_CRR to value 0"]
impl crate::Resettable for PWM_CRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

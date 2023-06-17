#[doc = "Register `PWM_CH3CR` reader"]
pub struct R(crate::R<PWM_CH3CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CH3CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CH3CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CH3CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CH3CR` writer"]
pub struct W(crate::W<PWM_CH3CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CH3CR_SPEC>;
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
impl From<crate::W<PWM_CH3CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CH3CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH3CV` reader - CH3CV"]
pub type CH3CV_R = crate::FieldReader<u16>;
#[doc = "Field `CH3CV` writer - CH3CV"]
pub type CH3CV_W<'a, const O: u8> = crate::FieldWriter<'a, PWM_CH3CR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CH3CV"]
    #[inline(always)]
    pub fn ch3cv(&self) -> CH3CV_R {
        CH3CV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH3CV"]
    #[inline(always)]
    #[must_use]
    pub fn ch3cv(&mut self) -> CH3CV_W<0> {
        CH3CV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM_CH3CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ch3cr](index.html) module"]
pub struct PWM_CH3CR_SPEC;
impl crate::RegisterSpec for PWM_CH3CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_ch3cr::R](R) reader structure"]
impl crate::Readable for PWM_CH3CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_ch3cr::W](W) writer structure"]
impl crate::Writable for PWM_CH3CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWM_CH3CR to value 0"]
impl crate::Resettable for PWM_CH3CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

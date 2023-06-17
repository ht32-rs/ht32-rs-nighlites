#[doc = "Register `CH5CR` reader"]
pub struct R(crate::R<CH5CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH5CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH5CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH5CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH5CR` writer"]
pub struct W(crate::W<CH5CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH5CR_SPEC>;
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
impl From<crate::W<CH5CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH5CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH5CV` reader - CH5CV"]
pub type CH5CV_R = crate::FieldReader<u16>;
#[doc = "Field `CH5CV` writer - CH5CV"]
pub type CH5CV_W<'a, const O: u8> = crate::FieldWriter<'a, CH5CR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CH5CV"]
    #[inline(always)]
    pub fn ch5cv(&self) -> CH5CV_R {
        CH5CV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH5CV"]
    #[inline(always)]
    #[must_use]
    pub fn ch5cv(&mut self) -> CH5CV_W<0> {
        CH5CV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH5CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5cr](index.html) module"]
pub struct CH5CR_SPEC;
impl crate::RegisterSpec for CH5CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch5cr::R](R) reader structure"]
impl crate::Readable for CH5CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch5cr::W](W) writer structure"]
impl crate::Writable for CH5CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH5CR to value 0"]
impl crate::Resettable for CH5CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

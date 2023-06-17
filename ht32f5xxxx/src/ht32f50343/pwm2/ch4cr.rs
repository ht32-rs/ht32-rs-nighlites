#[doc = "Register `CH4CR` reader"]
pub struct R(crate::R<CH4CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH4CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH4CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH4CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH4CR` writer"]
pub struct W(crate::W<CH4CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH4CR_SPEC>;
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
impl From<crate::W<CH4CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH4CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH4CV` reader - CH4CV"]
pub type CH4CV_R = crate::FieldReader<u16>;
#[doc = "Field `CH4CV` writer - CH4CV"]
pub type CH4CV_W<'a, const O: u8> = crate::FieldWriter<'a, CH4CR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CH4CV"]
    #[inline(always)]
    pub fn ch4cv(&self) -> CH4CV_R {
        CH4CV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH4CV"]
    #[inline(always)]
    #[must_use]
    pub fn ch4cv(&mut self) -> CH4CV_W<0> {
        CH4CV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH4CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4cr](index.html) module"]
pub struct CH4CR_SPEC;
impl crate::RegisterSpec for CH4CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch4cr::R](R) reader structure"]
impl crate::Readable for CH4CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch4cr::W](W) writer structure"]
impl crate::Writable for CH4CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH4CR to value 0"]
impl crate::Resettable for CH4CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `CH7CR` reader"]
pub struct R(crate::R<CH7CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH7CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH7CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH7CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH7CR` writer"]
pub struct W(crate::W<CH7CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH7CR_SPEC>;
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
impl From<crate::W<CH7CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH7CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH7CV` reader - CH7CV"]
pub type CH7CV_R = crate::FieldReader<u16>;
#[doc = "Field `CH7CV` writer - CH7CV"]
pub type CH7CV_W<'a, const O: u8> = crate::FieldWriter<'a, CH7CR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CH7CV"]
    #[inline(always)]
    pub fn ch7cv(&self) -> CH7CV_R {
        CH7CV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH7CV"]
    #[inline(always)]
    #[must_use]
    pub fn ch7cv(&mut self) -> CH7CV_W<0> {
        CH7CV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH7CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7cr](index.html) module"]
pub struct CH7CR_SPEC;
impl crate::RegisterSpec for CH7CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch7cr::R](R) reader structure"]
impl crate::Readable for CH7CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch7cr::W](W) writer structure"]
impl crate::Writable for CH7CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH7CR to value 0"]
impl crate::Resettable for CH7CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

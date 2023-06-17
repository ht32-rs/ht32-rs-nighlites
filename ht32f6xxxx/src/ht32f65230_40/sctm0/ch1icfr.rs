#[doc = "Register `CH1ICFR` reader"]
pub struct R(crate::R<CH1ICFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1ICFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1ICFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1ICFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1ICFR` writer"]
pub struct W(crate::W<CH1ICFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1ICFR_SPEC>;
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
impl From<crate::W<CH1ICFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1ICFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH1PSC` reader - CH1PSC"]
pub type CH1PSC_R = crate::FieldReader;
#[doc = "Field `CH1PSC` writer - CH1PSC"]
pub type CH1PSC_W<'a, const O: u8> = crate::FieldWriter<'a, CH1ICFR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 18:19 - CH1PSC"]
    #[inline(always)]
    pub fn ch1psc(&self) -> CH1PSC_R {
        CH1PSC_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - CH1PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ch1psc(&mut self) -> CH1PSC_W<18> {
        CH1PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH1ICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1icfr](index.html) module"]
pub struct CH1ICFR_SPEC;
impl crate::RegisterSpec for CH1ICFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1icfr::R](R) reader structure"]
impl crate::Readable for CH1ICFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1icfr::W](W) writer structure"]
impl crate::Writable for CH1ICFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH1ICFR to value 0"]
impl crate::Resettable for CH1ICFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

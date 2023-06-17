#[doc = "Register `RMR` reader"]
pub struct R(crate::R<RMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RMR` writer"]
pub struct W(crate::W<RMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RMR_SPEC>;
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
impl From<crate::W<RMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMR` reader - RMR"]
pub type RMR_R = crate::FieldReader<u32>;
#[doc = "Field `RMR` writer - RMR"]
pub type RMR_W<'a, const O: u8> = crate::FieldWriter<'a, RMR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - RMR"]
    #[inline(always)]
    pub fn rmr(&self) -> RMR_R {
        RMR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RMR"]
    #[inline(always)]
    #[must_use]
    pub fn rmr(&mut self) -> RMR_W<0> {
        RMR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmr](index.html) module"]
pub struct RMR_SPEC;
impl crate::RegisterSpec for RMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmr::R](R) reader structure"]
impl crate::Readable for RMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rmr::W](W) writer structure"]
impl crate::Writable for RMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMR to value 0"]
impl crate::Resettable for RMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

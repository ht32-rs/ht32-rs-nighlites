#[doc = "Register `RTCCMP` reader"]
pub struct R(crate::R<RTCCMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCMP` writer"]
pub struct W(crate::W<RTCCMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCMP_SPEC>;
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
impl From<crate::W<RTCCMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCCMPV` reader - RTCCMPV"]
pub type RTCCMPV_R = crate::FieldReader<u32>;
#[doc = "Field `RTCCMPV` writer - RTCCMPV"]
pub type RTCCMPV_W<'a, const O: u8> = crate::FieldWriter<'a, RTCCMP_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - RTCCMPV"]
    #[inline(always)]
    pub fn rtccmpv(&self) -> RTCCMPV_R {
        RTCCMPV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTCCMPV"]
    #[inline(always)]
    #[must_use]
    pub fn rtccmpv(&mut self) -> RTCCMPV_W<0> {
        RTCCMPV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCCMP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccmp](index.html) module"]
pub struct RTCCMP_SPEC;
impl crate::RegisterSpec for RTCCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtccmp::R](R) reader structure"]
impl crate::Readable for RTCCMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtccmp::W](W) writer structure"]
impl crate::Writable for RTCCMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCCMP to value 0"]
impl crate::Resettable for RTCCMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

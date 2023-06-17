#[doc = "Register `BAKTEST` reader"]
pub struct R(crate::R<BAKTEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAKTEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAKTEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAKTEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAKTEST` writer"]
pub struct W(crate::W<BAKTEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAKTEST_SPEC>;
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
impl From<crate::W<BAKTEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAKTEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAKTEST` reader - BAKTEST"]
pub type BAKTEST_R = crate::FieldReader;
#[doc = "Field `BAKTEST` writer - BAKTEST"]
pub type BAKTEST_W<'a, const O: u8> = crate::FieldWriter<'a, BAKTEST_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - BAKTEST"]
    #[inline(always)]
    pub fn baktest(&self) -> BAKTEST_R {
        BAKTEST_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - BAKTEST"]
    #[inline(always)]
    #[must_use]
    pub fn baktest(&mut self) -> BAKTEST_W<0> {
        BAKTEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BAKTEST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baktest](index.html) module"]
pub struct BAKTEST_SPEC;
impl crate::RegisterSpec for BAKTEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baktest::R](R) reader structure"]
impl crate::Readable for BAKTEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baktest::W](W) writer structure"]
impl crate::Writable for BAKTEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAKTEST to value 0"]
impl crate::Resettable for BAKTEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

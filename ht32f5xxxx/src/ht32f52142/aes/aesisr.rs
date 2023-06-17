#[doc = "Register `AESISR` reader"]
pub struct R(crate::R<AESISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESISR` writer"]
pub struct W(crate::W<AESISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESISR_SPEC>;
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
impl From<crate::W<AESISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IFINT` reader - IFINT"]
pub type IFINT_R = crate::BitReader;
#[doc = "Field `IFINT` writer - IFINT"]
pub type IFINT_W<'a, const O: u8> = crate::BitWriter<'a, AESISR_SPEC, O>;
#[doc = "Field `OFINT` reader - OFINT"]
pub type OFINT_R = crate::BitReader;
#[doc = "Field `OFINT` writer - OFINT"]
pub type OFINT_W<'a, const O: u8> = crate::BitWriter<'a, AESISR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - IFINT"]
    #[inline(always)]
    pub fn ifint(&self) -> IFINT_R {
        IFINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OFINT"]
    #[inline(always)]
    pub fn ofint(&self) -> OFINT_R {
        OFINT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IFINT"]
    #[inline(always)]
    #[must_use]
    pub fn ifint(&mut self) -> IFINT_W<0> {
        IFINT_W::new(self)
    }
    #[doc = "Bit 1 - OFINT"]
    #[inline(always)]
    #[must_use]
    pub fn ofint(&mut self) -> OFINT_W<1> {
        OFINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AESISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesisr](index.html) module"]
pub struct AESISR_SPEC;
impl crate::RegisterSpec for AESISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aesisr::R](R) reader structure"]
impl crate::Readable for AESISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesisr::W](W) writer structure"]
impl crate::Writable for AESISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESISR to value 0"]
impl crate::Resettable for AESISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

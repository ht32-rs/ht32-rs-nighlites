#[doc = "Register `PBINER` reader"]
pub struct R(crate::R<PBINER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBINER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBINER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBINER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBINER` writer"]
pub struct W(crate::W<PBINER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBINER_SPEC>;
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
impl From<crate::W<PBINER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBINER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBINEN` reader - PBINEN"]
pub type PBINEN_R = crate::FieldReader<u16>;
#[doc = "Field `PBINEN` writer - PBINEN"]
pub type PBINEN_W<'a, const O: u8> = crate::FieldWriter<'a, PBINER_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PBINEN"]
    #[inline(always)]
    pub fn pbinen(&self) -> PBINEN_R {
        PBINEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PBINEN"]
    #[inline(always)]
    #[must_use]
    pub fn pbinen(&mut self) -> PBINEN_W<0> {
        PBINEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBINER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbiner](index.html) module"]
pub struct PBINER_SPEC;
impl crate::RegisterSpec for PBINER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbiner::R](R) reader structure"]
impl crate::Readable for PBINER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbiner::W](W) writer structure"]
impl crate::Writable for PBINER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBINER to value 0"]
impl crate::Resettable for PBINER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

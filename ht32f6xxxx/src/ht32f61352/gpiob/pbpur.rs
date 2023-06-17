#[doc = "Register `PBPUR` reader"]
pub struct R(crate::R<PBPUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBPUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBPUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBPUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBPUR` writer"]
pub struct W(crate::W<PBPUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBPUR_SPEC>;
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
impl From<crate::W<PBPUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBPUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBPU` reader - PBPU"]
pub type PBPU_R = crate::FieldReader<u16>;
#[doc = "Field `PBPU` writer - PBPU"]
pub type PBPU_W<'a, const O: u8> = crate::FieldWriter<'a, PBPUR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PBPU"]
    #[inline(always)]
    pub fn pbpu(&self) -> PBPU_R {
        PBPU_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PBPU"]
    #[inline(always)]
    #[must_use]
    pub fn pbpu(&mut self) -> PBPU_W<0> {
        PBPU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBPUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbpur](index.html) module"]
pub struct PBPUR_SPEC;
impl crate::RegisterSpec for PBPUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbpur::R](R) reader structure"]
impl crate::Readable for PBPUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbpur::W](W) writer structure"]
impl crate::Writable for PBPUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBPUR to value 0"]
impl crate::Resettable for PBPUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `PCPUR` reader"]
pub struct R(crate::R<PCPUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCPUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCPUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCPUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCPUR` writer"]
pub struct W(crate::W<PCPUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCPUR_SPEC>;
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
impl From<crate::W<PCPUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCPUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCPU` reader - PCPU"]
pub type PCPU_R = crate::FieldReader<u16>;
#[doc = "Field `PCPU` writer - PCPU"]
pub type PCPU_W<'a, const O: u8> = crate::FieldWriter<'a, PCPUR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PCPU"]
    #[inline(always)]
    pub fn pcpu(&self) -> PCPU_R {
        PCPU_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PCPU"]
    #[inline(always)]
    #[must_use]
    pub fn pcpu(&mut self) -> PCPU_W<0> {
        PCPU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCPUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcpur](index.html) module"]
pub struct PCPUR_SPEC;
impl crate::RegisterSpec for PCPUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcpur::R](R) reader structure"]
impl crate::Readable for PCPUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcpur::W](W) writer structure"]
impl crate::Writable for PCPUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCPUR to value 0"]
impl crate::Resettable for PCPUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

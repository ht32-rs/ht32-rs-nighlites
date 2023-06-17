#[doc = "Register `PAPUR` reader"]
pub struct R(crate::R<PAPUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAPUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAPUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAPUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAPUR` writer"]
pub struct W(crate::W<PAPUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAPUR_SPEC>;
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
impl From<crate::W<PAPUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAPUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAPU` reader - PAPU"]
pub type PAPU_R = crate::FieldReader<u16>;
#[doc = "Field `PAPU` writer - PAPU"]
pub type PAPU_W<'a, const O: u8> = crate::FieldWriter<'a, PAPUR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PAPU"]
    #[inline(always)]
    pub fn papu(&self) -> PAPU_R {
        PAPU_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PAPU"]
    #[inline(always)]
    #[must_use]
    pub fn papu(&mut self) -> PAPU_W<0> {
        PAPU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAPUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [papur](index.html) module"]
pub struct PAPUR_SPEC;
impl crate::RegisterSpec for PAPUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [papur::R](R) reader structure"]
impl crate::Readable for PAPUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [papur::W](W) writer structure"]
impl crate::Writable for PAPUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAPUR to value 0"]
impl crate::Resettable for PAPUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

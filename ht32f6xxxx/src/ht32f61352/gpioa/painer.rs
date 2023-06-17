#[doc = "Register `PAINER` reader"]
pub struct R(crate::R<PAINER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAINER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAINER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAINER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAINER` writer"]
pub struct W(crate::W<PAINER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAINER_SPEC>;
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
impl From<crate::W<PAINER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAINER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAINEN` reader - PAINEN"]
pub type PAINEN_R = crate::FieldReader<u16>;
#[doc = "Field `PAINEN` writer - PAINEN"]
pub type PAINEN_W<'a, const O: u8> = crate::FieldWriter<'a, PAINER_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PAINEN"]
    #[inline(always)]
    pub fn painen(&self) -> PAINEN_R {
        PAINEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PAINEN"]
    #[inline(always)]
    #[must_use]
    pub fn painen(&mut self) -> PAINEN_W<0> {
        PAINEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAINER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [painer](index.html) module"]
pub struct PAINER_SPEC;
impl crate::RegisterSpec for PAINER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [painer::R](R) reader structure"]
impl crate::Readable for PAINER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [painer::W](W) writer structure"]
impl crate::Writable for PAINER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAINER to value 0"]
impl crate::Resettable for PAINER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

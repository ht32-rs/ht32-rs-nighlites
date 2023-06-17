#[doc = "Register `PCINER` reader"]
pub struct R(crate::R<PCINER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCINER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCINER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCINER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCINER` writer"]
pub struct W(crate::W<PCINER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCINER_SPEC>;
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
impl From<crate::W<PCINER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCINER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCINEN` reader - PCINEN"]
pub type PCINEN_R = crate::FieldReader<u16>;
#[doc = "Field `PCINEN` writer - PCINEN"]
pub type PCINEN_W<'a, const O: u8> = crate::FieldWriter<'a, PCINER_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PCINEN"]
    #[inline(always)]
    pub fn pcinen(&self) -> PCINEN_R {
        PCINEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PCINEN"]
    #[inline(always)]
    #[must_use]
    pub fn pcinen(&mut self) -> PCINEN_W<0> {
        PCINEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCINER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pciner](index.html) module"]
pub struct PCINER_SPEC;
impl crate::RegisterSpec for PCINER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pciner::R](R) reader structure"]
impl crate::Readable for PCINER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pciner::W](W) writer structure"]
impl crate::Writable for PCINER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCINER to value 0"]
impl crate::Resettable for PCINER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

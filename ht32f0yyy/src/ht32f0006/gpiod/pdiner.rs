#[doc = "Register `PDINER` reader"]
pub struct R(crate::R<PDINER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDINER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDINER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDINER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDINER` writer"]
pub struct W(crate::W<PDINER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDINER_SPEC>;
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
impl From<crate::W<PDINER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDINER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDINEN` reader - PDINEN"]
pub type PDINEN_R = crate::FieldReader;
#[doc = "Field `PDINEN` writer - PDINEN"]
pub type PDINEN_W<'a, const O: u8> = crate::FieldWriter<'a, PDINER_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PDINEN"]
    #[inline(always)]
    pub fn pdinen(&self) -> PDINEN_R {
        PDINEN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDINEN"]
    #[inline(always)]
    #[must_use]
    pub fn pdinen(&mut self) -> PDINEN_W<0> {
        PDINEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDINER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdiner](index.html) module"]
pub struct PDINER_SPEC;
impl crate::RegisterSpec for PDINER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdiner::R](R) reader structure"]
impl crate::Readable for PDINER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdiner::W](W) writer structure"]
impl crate::Writable for PDINER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDINER to value 0"]
impl crate::Resettable for PDINER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

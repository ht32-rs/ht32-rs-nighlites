#[doc = "Register `PCSRR` reader"]
pub struct R(crate::R<PCSRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCSRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCSRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCSRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCSRR` writer"]
pub struct W(crate::W<PCSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCSRR_SPEC>;
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
impl From<crate::W<PCSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCSRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCSET` reader - PCSET"]
pub type PCSET_R = crate::FieldReader<u16>;
#[doc = "Field `PCSET` writer - PCSET"]
pub type PCSET_W<'a, const O: u8> = crate::FieldWriter<'a, PCSRR_SPEC, 16, O, u16>;
#[doc = "Field `PCRST` reader - PCRST"]
pub type PCRST_R = crate::FieldReader<u16>;
#[doc = "Field `PCRST` writer - PCRST"]
pub type PCRST_W<'a, const O: u8> = crate::FieldWriter<'a, PCSRR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PCSET"]
    #[inline(always)]
    pub fn pcset(&self) -> PCSET_R {
        PCSET_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PCRST"]
    #[inline(always)]
    pub fn pcrst(&self) -> PCRST_R {
        PCRST_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PCSET"]
    #[inline(always)]
    #[must_use]
    pub fn pcset(&mut self) -> PCSET_W<0> {
        PCSET_W::new(self)
    }
    #[doc = "Bits 16:31 - PCRST"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst(&mut self) -> PCRST_W<16> {
        PCRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCSRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsrr](index.html) module"]
pub struct PCSRR_SPEC;
impl crate::RegisterSpec for PCSRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcsrr::R](R) reader structure"]
impl crate::Readable for PCSRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcsrr::W](W) writer structure"]
impl crate::Writable for PCSRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCSRR to value 0"]
impl crate::Resettable for PCSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

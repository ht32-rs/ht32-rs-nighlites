#[doc = "Register `CFSR` reader"]
pub struct R(crate::R<CFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFSR` writer"]
pub struct W(crate::W<CFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFSR_SPEC>;
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
impl From<crate::W<CFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MFSR` reader - MFSR"]
pub type MFSR_R = crate::FieldReader;
#[doc = "Field `MFSR` writer - MFSR"]
pub type MFSR_W<'a, const O: u8> = crate::FieldWriter<'a, CFSR_SPEC, 8, O>;
#[doc = "Field `BFSR` reader - BFSR"]
pub type BFSR_R = crate::FieldReader;
#[doc = "Field `BFSR` writer - BFSR"]
pub type BFSR_W<'a, const O: u8> = crate::FieldWriter<'a, CFSR_SPEC, 8, O>;
#[doc = "Field `UFSR` reader - UFSR"]
pub type UFSR_R = crate::FieldReader<u16>;
#[doc = "Field `UFSR` writer - UFSR"]
pub type UFSR_W<'a, const O: u8> = crate::FieldWriter<'a, CFSR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:7 - MFSR"]
    #[inline(always)]
    pub fn mfsr(&self) -> MFSR_R {
        MFSR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - BFSR"]
    #[inline(always)]
    pub fn bfsr(&self) -> BFSR_R {
        BFSR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - UFSR"]
    #[inline(always)]
    pub fn ufsr(&self) -> UFSR_R {
        UFSR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - MFSR"]
    #[inline(always)]
    #[must_use]
    pub fn mfsr(&mut self) -> MFSR_W<0> {
        MFSR_W::new(self)
    }
    #[doc = "Bits 8:15 - BFSR"]
    #[inline(always)]
    #[must_use]
    pub fn bfsr(&mut self) -> BFSR_W<8> {
        BFSR_W::new(self)
    }
    #[doc = "Bits 16:31 - UFSR"]
    #[inline(always)]
    #[must_use]
    pub fn ufsr(&mut self) -> UFSR_W<16> {
        UFSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfsr](index.html) module"]
pub struct CFSR_SPEC;
impl crate::RegisterSpec for CFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfsr::R](R) reader structure"]
impl crate::Readable for CFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfsr::W](W) writer structure"]
impl crate::Writable for CFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFSR to value 0"]
impl crate::Resettable for CFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

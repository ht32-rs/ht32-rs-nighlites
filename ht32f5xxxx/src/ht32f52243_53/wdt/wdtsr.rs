#[doc = "Register `WDTSR` reader"]
pub struct R(crate::R<WDTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTSR` writer"]
pub struct W(crate::W<WDTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTSR_SPEC>;
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
impl From<crate::W<WDTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTUF` reader - WDTUF"]
pub type WDTUF_R = crate::BitReader;
#[doc = "Field `WDTUF` writer - WDTUF"]
pub type WDTUF_W<'a, const O: u8> = crate::BitWriter<'a, WDTSR_SPEC, O>;
#[doc = "Field `WDTERR` reader - WDTERR"]
pub type WDTERR_R = crate::BitReader;
#[doc = "Field `WDTERR` writer - WDTERR"]
pub type WDTERR_W<'a, const O: u8> = crate::BitWriter<'a, WDTSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - WDTUF"]
    #[inline(always)]
    pub fn wdtuf(&self) -> WDTUF_R {
        WDTUF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDTERR"]
    #[inline(always)]
    pub fn wdterr(&self) -> WDTERR_R {
        WDTERR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDTUF"]
    #[inline(always)]
    #[must_use]
    pub fn wdtuf(&mut self) -> WDTUF_W<0> {
        WDTUF_W::new(self)
    }
    #[doc = "Bit 1 - WDTERR"]
    #[inline(always)]
    #[must_use]
    pub fn wdterr(&mut self) -> WDTERR_W<1> {
        WDTERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtsr](index.html) module"]
pub struct WDTSR_SPEC;
impl crate::RegisterSpec for WDTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtsr::R](R) reader structure"]
impl crate::Readable for WDTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtsr::W](W) writer structure"]
impl crate::Writable for WDTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTSR to value 0"]
impl crate::Resettable for WDTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

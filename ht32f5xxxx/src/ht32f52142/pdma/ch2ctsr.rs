#[doc = "Register `CH2CTSR` reader"]
pub struct R(crate::R<CH2CTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2CTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2CTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2CTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2CTSR` writer"]
pub struct W(crate::W<CH2CTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2CTSR_SPEC>;
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
impl From<crate::W<CH2CTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2CTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CBLKCNT` reader - CBLKCNT"]
pub type CBLKCNT_R = crate::FieldReader<u16>;
#[doc = "Field `CBLKCNT` writer - CBLKCNT"]
pub type CBLKCNT_W<'a, const O: u8> = crate::FieldWriter<'a, CH2CTSR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 16:31 - CBLKCNT"]
    #[inline(always)]
    pub fn cblkcnt(&self) -> CBLKCNT_R {
        CBLKCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - CBLKCNT"]
    #[inline(always)]
    #[must_use]
    pub fn cblkcnt(&mut self) -> CBLKCNT_W<16> {
        CBLKCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA_CH2CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2ctsr](index.html) module"]
pub struct CH2CTSR_SPEC;
impl crate::RegisterSpec for CH2CTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2ctsr::R](R) reader structure"]
impl crate::Readable for CH2CTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2ctsr::W](W) writer structure"]
impl crate::Writable for CH2CTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH2CTSR to value 0"]
impl crate::Resettable for CH2CTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

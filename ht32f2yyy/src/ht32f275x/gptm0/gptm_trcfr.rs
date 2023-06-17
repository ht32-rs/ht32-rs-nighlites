#[doc = "Register `GPTM_TRCFR` reader"]
pub struct R(crate::R<GPTM_TRCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTM_TRCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTM_TRCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTM_TRCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTM_TRCFR` writer"]
pub struct W(crate::W<GPTM_TRCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTM_TRCFR_SPEC>;
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
impl From<crate::W<GPTM_TRCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTM_TRCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRSEL` reader - TRSEL"]
pub type TRSEL_R = crate::FieldReader;
#[doc = "Field `TRSEL` writer - TRSEL"]
pub type TRSEL_W<'a, const O: u8> = crate::FieldWriter<'a, GPTM_TRCFR_SPEC, 4, O>;
#[doc = "Field `ETF` reader - ETF"]
pub type ETF_R = crate::FieldReader;
#[doc = "Field `ETF` writer - ETF"]
pub type ETF_W<'a, const O: u8> = crate::FieldWriter<'a, GPTM_TRCFR_SPEC, 4, O>;
#[doc = "Field `ETIPSC` reader - ETIPSC"]
pub type ETIPSC_R = crate::FieldReader;
#[doc = "Field `ETIPSC` writer - ETIPSC"]
pub type ETIPSC_W<'a, const O: u8> = crate::FieldWriter<'a, GPTM_TRCFR_SPEC, 2, O>;
#[doc = "Field `ETIPOL` reader - ETIPOL"]
pub type ETIPOL_R = crate::BitReader;
#[doc = "Field `ETIPOL` writer - ETIPOL"]
pub type ETIPOL_W<'a, const O: u8> = crate::BitWriter<'a, GPTM_TRCFR_SPEC, O>;
#[doc = "Field `ECME` reader - ECME"]
pub type ECME_R = crate::BitReader;
#[doc = "Field `ECME` writer - ECME"]
pub type ECME_W<'a, const O: u8> = crate::BitWriter<'a, GPTM_TRCFR_SPEC, O>;
impl R {
    #[doc = "Bits 0:3 - TRSEL"]
    #[inline(always)]
    pub fn trsel(&self) -> TRSEL_R {
        TRSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ETF"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - ETIPSC"]
    #[inline(always)]
    pub fn etipsc(&self) -> ETIPSC_R {
        ETIPSC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - ETIPOL"]
    #[inline(always)]
    pub fn etipol(&self) -> ETIPOL_R {
        ETIPOL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - ECME"]
    #[inline(always)]
    pub fn ecme(&self) -> ECME_R {
        ECME_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TRSEL"]
    #[inline(always)]
    #[must_use]
    pub fn trsel(&mut self) -> TRSEL_W<0> {
        TRSEL_W::new(self)
    }
    #[doc = "Bits 8:11 - ETF"]
    #[inline(always)]
    #[must_use]
    pub fn etf(&mut self) -> ETF_W<8> {
        ETF_W::new(self)
    }
    #[doc = "Bits 12:13 - ETIPSC"]
    #[inline(always)]
    #[must_use]
    pub fn etipsc(&mut self) -> ETIPSC_W<12> {
        ETIPSC_W::new(self)
    }
    #[doc = "Bit 16 - ETIPOL"]
    #[inline(always)]
    #[must_use]
    pub fn etipol(&mut self) -> ETIPOL_W<16> {
        ETIPOL_W::new(self)
    }
    #[doc = "Bit 24 - ECME"]
    #[inline(always)]
    #[must_use]
    pub fn ecme(&mut self) -> ECME_W<24> {
        ECME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM_TRCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptm_trcfr](index.html) module"]
pub struct GPTM_TRCFR_SPEC;
impl crate::RegisterSpec for GPTM_TRCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gptm_trcfr::R](R) reader structure"]
impl crate::Readable for GPTM_TRCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptm_trcfr::W](W) writer structure"]
impl crate::Writable for GPTM_TRCFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPTM_TRCFR to value 0"]
impl crate::Resettable for GPTM_TRCFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

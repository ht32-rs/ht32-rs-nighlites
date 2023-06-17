#[doc = "Register `RSTCU_APBPRSTR1` reader"]
pub struct R(crate::R<RSTCU_APBPRSTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCU_APBPRSTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCU_APBPRSTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCU_APBPRSTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCU_APBPRSTR1` writer"]
pub struct W(crate::W<RSTCU_APBPRSTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCU_APBPRSTR1_SPEC>;
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
impl From<crate::W<RSTCU_APBPRSTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCU_APBPRSTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTRST` reader - WDTRST"]
pub type WDTRST_R = crate::BitReader;
#[doc = "Field `WDTRST` writer - WDTRST"]
pub type WDTRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR1_SPEC, O>;
#[doc = "Field `GPTM0RST` reader - GPTM0RST"]
pub type GPTM0RST_R = crate::BitReader;
#[doc = "Field `GPTM0RST` writer - GPTM0RST"]
pub type GPTM0RST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR1_SPEC, O>;
#[doc = "Field `GPTM1RST` reader - GPTM1RST"]
pub type GPTM1RST_R = crate::BitReader;
#[doc = "Field `GPTM1RST` writer - GPTM1RST"]
pub type GPTM1RST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR1_SPEC, O>;
#[doc = "Field `OPA0RST` reader - OPA0RST"]
pub type OPA0RST_R = crate::BitReader;
#[doc = "Field `OPA0RST` writer - OPA0RST"]
pub type OPA0RST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR1_SPEC, O>;
#[doc = "Field `OPA1RST` reader - OPA1RST"]
pub type OPA1RST_R = crate::BitReader;
#[doc = "Field `OPA1RST` writer - OPA1RST"]
pub type OPA1RST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR1_SPEC, O>;
#[doc = "Field `ADCRST` reader - ADCRST"]
pub type ADCRST_R = crate::BitReader;
#[doc = "Field `ADCRST` writer - ADCRST"]
pub type ADCRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_APBPRSTR1_SPEC, O>;
impl R {
    #[doc = "Bit 4 - WDTRST"]
    #[inline(always)]
    pub fn wdtrst(&self) -> WDTRST_R {
        WDTRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM0RST"]
    #[inline(always)]
    pub fn gptm0rst(&self) -> GPTM0RST_R {
        GPTM0RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM1RST"]
    #[inline(always)]
    pub fn gptm1rst(&self) -> GPTM1RST_R {
        GPTM1RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 22 - OPA0RST"]
    #[inline(always)]
    pub fn opa0rst(&self) -> OPA0RST_R {
        OPA0RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OPA1RST"]
    #[inline(always)]
    pub fn opa1rst(&self) -> OPA1RST_R {
        OPA1RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ADCRST"]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - WDTRST"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrst(&mut self) -> WDTRST_W<4> {
        WDTRST_W::new(self)
    }
    #[doc = "Bit 8 - GPTM0RST"]
    #[inline(always)]
    #[must_use]
    pub fn gptm0rst(&mut self) -> GPTM0RST_W<8> {
        GPTM0RST_W::new(self)
    }
    #[doc = "Bit 9 - GPTM1RST"]
    #[inline(always)]
    #[must_use]
    pub fn gptm1rst(&mut self) -> GPTM1RST_W<9> {
        GPTM1RST_W::new(self)
    }
    #[doc = "Bit 22 - OPA0RST"]
    #[inline(always)]
    #[must_use]
    pub fn opa0rst(&mut self) -> OPA0RST_W<22> {
        OPA0RST_W::new(self)
    }
    #[doc = "Bit 23 - OPA1RST"]
    #[inline(always)]
    #[must_use]
    pub fn opa1rst(&mut self) -> OPA1RST_W<23> {
        OPA1RST_W::new(self)
    }
    #[doc = "Bit 24 - ADCRST"]
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<24> {
        ADCRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RSTCU_APBPRSTR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcu_apbprstr1](index.html) module"]
pub struct RSTCU_APBPRSTR1_SPEC;
impl crate::RegisterSpec for RSTCU_APBPRSTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstcu_apbprstr1::R](R) reader structure"]
impl crate::Readable for RSTCU_APBPRSTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstcu_apbprstr1::W](W) writer structure"]
impl crate::Writable for RSTCU_APBPRSTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTCU_APBPRSTR1 to value 0"]
impl crate::Resettable for RSTCU_APBPRSTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

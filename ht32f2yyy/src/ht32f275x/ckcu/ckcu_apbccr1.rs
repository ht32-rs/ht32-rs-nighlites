#[doc = "Register `CKCU_APBCCR1` reader"]
pub struct R(crate::R<CKCU_APBCCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_APBCCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_APBCCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_APBCCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_APBCCR1` writer"]
pub struct W(crate::W<CKCU_APBCCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_APBCCR1_SPEC>;
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
impl From<crate::W<CKCU_APBCCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_APBCCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCTMEN` reader - MCTMEN"]
pub type MCTMEN_R = crate::BitReader;
#[doc = "Field `MCTMEN` writer - MCTMEN"]
pub type MCTMEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `WDTEN` reader - WDTEN"]
pub type WDTEN_R = crate::BitReader;
#[doc = "Field `WDTEN` writer - WDTEN"]
pub type WDTEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `RTCEN` reader - RTCEN"]
pub type RTCEN_R = crate::BitReader;
#[doc = "Field `RTCEN` writer - RTCEN"]
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `GPTM0EN` reader - GPTM0EN"]
pub type GPTM0EN_R = crate::BitReader;
#[doc = "Field `GPTM0EN` writer - GPTM0EN"]
pub type GPTM0EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `GPTM1EN` reader - GPTM1EN"]
pub type GPTM1EN_R = crate::BitReader;
#[doc = "Field `GPTM1EN` writer - GPTM1EN"]
pub type GPTM1EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `USBEN` reader - USBEN"]
pub type USBEN_R = crate::BitReader;
#[doc = "Field `USBEN` writer - USBEN"]
pub type USBEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `BFTM0EN` reader - BFTM0EN"]
pub type BFTM0EN_R = crate::BitReader;
#[doc = "Field `BFTM0EN` writer - BFTM0EN"]
pub type BFTM0EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `BFTM1EN` reader - BFTM1EN"]
pub type BFTM1EN_R = crate::BitReader;
#[doc = "Field `BFTM1EN` writer - BFTM1EN"]
pub type BFTM1EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `OPA0EN` reader - OPA0EN"]
pub type OPA0EN_R = crate::BitReader;
#[doc = "Field `OPA0EN` writer - OPA0EN"]
pub type OPA0EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `OPA1EN` reader - OPA1EN"]
pub type OPA1EN_R = crate::BitReader;
#[doc = "Field `OPA1EN` writer - OPA1EN"]
pub type OPA1EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `ADCEN` reader - ADCEN"]
pub type ADCEN_R = crate::BitReader;
#[doc = "Field `ADCEN` writer - ADCEN"]
pub type ADCEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - MCTMEN"]
    #[inline(always)]
    pub fn mctmen(&self) -> MCTMEN_R {
        MCTMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - WDTEN"]
    #[inline(always)]
    pub fn wdten(&self) -> WDTEN_R {
        WDTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - RTCEN"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM0EN"]
    #[inline(always)]
    pub fn gptm0en(&self) -> GPTM0EN_R {
        GPTM0EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM1EN"]
    #[inline(always)]
    pub fn gptm1en(&self) -> GPTM1EN_R {
        GPTM1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - USBEN"]
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - BFTM0EN"]
    #[inline(always)]
    pub fn bftm0en(&self) -> BFTM0EN_R {
        BFTM0EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BFTM1EN"]
    #[inline(always)]
    pub fn bftm1en(&self) -> BFTM1EN_R {
        BFTM1EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 22 - OPA0EN"]
    #[inline(always)]
    pub fn opa0en(&self) -> OPA0EN_R {
        OPA0EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OPA1EN"]
    #[inline(always)]
    pub fn opa1en(&self) -> OPA1EN_R {
        OPA1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ADCEN"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCTMEN"]
    #[inline(always)]
    #[must_use]
    pub fn mctmen(&mut self) -> MCTMEN_W<0> {
        MCTMEN_W::new(self)
    }
    #[doc = "Bit 4 - WDTEN"]
    #[inline(always)]
    #[must_use]
    pub fn wdten(&mut self) -> WDTEN_W<4> {
        WDTEN_W::new(self)
    }
    #[doc = "Bit 6 - RTCEN"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<6> {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 8 - GPTM0EN"]
    #[inline(always)]
    #[must_use]
    pub fn gptm0en(&mut self) -> GPTM0EN_W<8> {
        GPTM0EN_W::new(self)
    }
    #[doc = "Bit 9 - GPTM1EN"]
    #[inline(always)]
    #[must_use]
    pub fn gptm1en(&mut self) -> GPTM1EN_W<9> {
        GPTM1EN_W::new(self)
    }
    #[doc = "Bit 14 - USBEN"]
    #[inline(always)]
    #[must_use]
    pub fn usben(&mut self) -> USBEN_W<14> {
        USBEN_W::new(self)
    }
    #[doc = "Bit 16 - BFTM0EN"]
    #[inline(always)]
    #[must_use]
    pub fn bftm0en(&mut self) -> BFTM0EN_W<16> {
        BFTM0EN_W::new(self)
    }
    #[doc = "Bit 17 - BFTM1EN"]
    #[inline(always)]
    #[must_use]
    pub fn bftm1en(&mut self) -> BFTM1EN_W<17> {
        BFTM1EN_W::new(self)
    }
    #[doc = "Bit 22 - OPA0EN"]
    #[inline(always)]
    #[must_use]
    pub fn opa0en(&mut self) -> OPA0EN_W<22> {
        OPA0EN_W::new(self)
    }
    #[doc = "Bit 23 - OPA1EN"]
    #[inline(always)]
    #[must_use]
    pub fn opa1en(&mut self) -> OPA1EN_W<23> {
        OPA1EN_W::new(self)
    }
    #[doc = "Bit 24 - ADCEN"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<24> {
        ADCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_APBCCR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_apbccr1](index.html) module"]
pub struct CKCU_APBCCR1_SPEC;
impl crate::RegisterSpec for CKCU_APBCCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_apbccr1::R](R) reader structure"]
impl crate::Readable for CKCU_APBCCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_apbccr1::W](W) writer structure"]
impl crate::Writable for CKCU_APBCCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_APBCCR1 to value 0"]
impl crate::Resettable for CKCU_APBCCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

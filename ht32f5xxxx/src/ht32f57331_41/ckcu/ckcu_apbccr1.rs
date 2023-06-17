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
#[doc = "Field `WDTEN` reader - WDTEN"]
pub type WDTEN_R = crate::BitReader;
#[doc = "Field `WDTEN` writer - WDTEN"]
pub type WDTEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `VDDREN` reader - VDDREN"]
pub type VDDREN_R = crate::BitReader;
#[doc = "Field `VDDREN` writer - VDDREN"]
pub type VDDREN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `GPTM0EN` reader - GPTM0EN"]
pub type GPTM0EN_R = crate::BitReader;
#[doc = "Field `GPTM0EN` writer - GPTM0EN"]
pub type GPTM0EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `PWM0EN` reader - PWM0EN"]
pub type PWM0EN_R = crate::BitReader;
#[doc = "Field `PWM0EN` writer - PWM0EN"]
pub type PWM0EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `PWM1EN` reader - PWM1EN"]
pub type PWM1EN_R = crate::BitReader;
#[doc = "Field `PWM1EN` writer - PWM1EN"]
pub type PWM1EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `BFTM0EN` reader - BFTM0EN"]
pub type BFTM0EN_R = crate::BitReader;
#[doc = "Field `BFTM0EN` writer - BFTM0EN"]
pub type BFTM0EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `BFTM1EN` reader - BFTM1EN"]
pub type BFTM1EN_R = crate::BitReader;
#[doc = "Field `BFTM1EN` writer - BFTM1EN"]
pub type BFTM1EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `LCDREN` reader - LCDREN"]
pub type LCDREN_R = crate::BitReader;
#[doc = "Field `LCDREN` writer - LCDREN"]
pub type LCDREN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `LCDCEN` reader - LCDCEN"]
pub type LCDCEN_R = crate::BitReader;
#[doc = "Field `LCDCEN` writer - LCDCEN"]
pub type LCDCEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `DACCEN` reader - DACCEN"]
pub type DACCEN_R = crate::BitReader;
#[doc = "Field `DACCEN` writer - DACCEN"]
pub type DACCEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `CMPEN` reader - CMPEN"]
pub type CMPEN_R = crate::BitReader;
#[doc = "Field `CMPEN` writer - CMPEN"]
pub type CMPEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `ADCEN` reader - ADCEN"]
pub type ADCEN_R = crate::BitReader;
#[doc = "Field `ADCEN` writer - ADCEN"]
pub type ADCEN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `SCTM0EN` reader - SCTM0EN"]
pub type SCTM0EN_R = crate::BitReader;
#[doc = "Field `SCTM0EN` writer - SCTM0EN"]
pub type SCTM0EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
#[doc = "Field `SCTM1EN` reader - SCTM1EN"]
pub type SCTM1EN_R = crate::BitReader;
#[doc = "Field `SCTM1EN` writer - SCTM1EN"]
pub type SCTM1EN_W<'a, const O: u8> = crate::BitWriter<'a, CKCU_APBCCR1_SPEC, O>;
impl R {
    #[doc = "Bit 4 - WDTEN"]
    #[inline(always)]
    pub fn wdten(&self) -> WDTEN_R {
        WDTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - VDDREN"]
    #[inline(always)]
    pub fn vddren(&self) -> VDDREN_R {
        VDDREN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM0EN"]
    #[inline(always)]
    pub fn gptm0en(&self) -> GPTM0EN_R {
        GPTM0EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - PWM0EN"]
    #[inline(always)]
    pub fn pwm0en(&self) -> PWM0EN_R {
        PWM0EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PWM1EN"]
    #[inline(always)]
    pub fn pwm1en(&self) -> PWM1EN_R {
        PWM1EN_R::new(((self.bits >> 13) & 1) != 0)
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
    #[doc = "Bit 19 - LCDREN"]
    #[inline(always)]
    pub fn lcdren(&self) -> LCDREN_R {
        LCDREN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LCDCEN"]
    #[inline(always)]
    pub fn lcdcen(&self) -> LCDCEN_R {
        LCDCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DACCEN"]
    #[inline(always)]
    pub fn daccen(&self) -> DACCEN_R {
        DACCEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CMPEN"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - ADCEN"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - SCTM0EN"]
    #[inline(always)]
    pub fn sctm0en(&self) -> SCTM0EN_R {
        SCTM0EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCTM1EN"]
    #[inline(always)]
    pub fn sctm1en(&self) -> SCTM1EN_R {
        SCTM1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - WDTEN"]
    #[inline(always)]
    #[must_use]
    pub fn wdten(&mut self) -> WDTEN_W<4> {
        WDTEN_W::new(self)
    }
    #[doc = "Bit 6 - VDDREN"]
    #[inline(always)]
    #[must_use]
    pub fn vddren(&mut self) -> VDDREN_W<6> {
        VDDREN_W::new(self)
    }
    #[doc = "Bit 8 - GPTM0EN"]
    #[inline(always)]
    #[must_use]
    pub fn gptm0en(&mut self) -> GPTM0EN_W<8> {
        GPTM0EN_W::new(self)
    }
    #[doc = "Bit 12 - PWM0EN"]
    #[inline(always)]
    #[must_use]
    pub fn pwm0en(&mut self) -> PWM0EN_W<12> {
        PWM0EN_W::new(self)
    }
    #[doc = "Bit 13 - PWM1EN"]
    #[inline(always)]
    #[must_use]
    pub fn pwm1en(&mut self) -> PWM1EN_W<13> {
        PWM1EN_W::new(self)
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
    #[doc = "Bit 19 - LCDREN"]
    #[inline(always)]
    #[must_use]
    pub fn lcdren(&mut self) -> LCDREN_W<19> {
        LCDREN_W::new(self)
    }
    #[doc = "Bit 20 - LCDCEN"]
    #[inline(always)]
    #[must_use]
    pub fn lcdcen(&mut self) -> LCDCEN_W<20> {
        LCDCEN_W::new(self)
    }
    #[doc = "Bit 21 - DACCEN"]
    #[inline(always)]
    #[must_use]
    pub fn daccen(&mut self) -> DACCEN_W<21> {
        DACCEN_W::new(self)
    }
    #[doc = "Bit 22 - CMPEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CMPEN_W<22> {
        CMPEN_W::new(self)
    }
    #[doc = "Bit 24 - ADCEN"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<24> {
        ADCEN_W::new(self)
    }
    #[doc = "Bit 28 - SCTM0EN"]
    #[inline(always)]
    #[must_use]
    pub fn sctm0en(&mut self) -> SCTM0EN_W<28> {
        SCTM0EN_W::new(self)
    }
    #[doc = "Bit 29 - SCTM1EN"]
    #[inline(always)]
    #[must_use]
    pub fn sctm1en(&mut self) -> SCTM1EN_W<29> {
        SCTM1EN_W::new(self)
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
